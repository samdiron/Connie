use std::io::Result;
use std::net::SocketAddr;
use std::time::Duration;
use std::thread::{self};
use std::process::exit;
use common_lib::public_ip;
use lib_db::sqlite::sqlite_host::SqliteHost;
use lib_db::sqlite::sqlite_user::{ShortUser, SqliteUser};
use lib_db::types::SqlitePool;
use lib_db::sqlite::sqlite_jwt::{add_jwt, delete_jwt};
use common_lib::log::{debug, info, warn, error};
use common_lib::tokio::io::{
    AsyncReadExt,
    AsyncWriteExt
};
use common_lib::tokio::net::TcpStream;
use crate::client::client::Connection;
use crate::common::handshakes;
use crate::common::request::{
    JWT_AUTH, RQM, SIGNIN_CRED, UNAUTHORIZED
};
use crate::common::request::req_format::{JwtReq, LoginReq};
use crate::common::util::{rvfs, wvts};
use crate::types::LOGIN_CRED;
use super::handle_request::handle_client_request;



pub async fn signup_process(
    addr: SocketAddr,
    user: ShortUser,
    pool: &SqlitePool
) -> Result<()> {
    let short_user_vec = user.sz().unwrap();
    let mut stream = TcpStream::connect(&addr).await?;
    info!("connected to {addr}");
    stream.write_u8(SIGNIN_CRED).await?;
    stream.flush().await?;
    debug!("sent status to host {SIGNIN_CRED}");
    let will_allow = stream.read_u8().await?;
    if will_allow == 0 {
        info!("server accsepted request");
        let vector = rvfs(&mut stream).await?;
        let server = SqliteHost::dz(vector).unwrap();
        info!("server: name: {}, host: {};",&server.name, &server.host);
        info!("the thread will pause for 2s if you want to cancel type ^c");
        let dur = Duration::from_secs(2);
        thread::sleep(dur);
        stream.write_u8(0).await?;
        stream.flush().await?;
        wvts(&mut stream, short_user_vec).await?;

        let user_vec = rvfs(&mut stream).await?;
        
        let mut user = SqliteUser::dz(user_vec).unwrap();
        user.host = server.cpid.clone();
        debug!("host name: {}",&user.host);
        SqliteUser::add_user(user, pool).await.unwrap();
        SqliteHost::new(server, pool).await;

    }
    else {
        info!("server did not allow to signup");
    }

    Ok(())
    
}


pub async fn connect_tcp(
    pool: &SqlitePool,
    conn: Connection,
    check_for_sum: Option<bool>,
    rqm: RQM
) -> Result<u8> {
    let port = conn.server.port;

    let pub_addr = SocketAddr::new(conn.server.pub_ip.parse().unwrap(), port);
    
    let pri_addr = SocketAddr::new(conn.server.pri_ip.parse().unwrap(), port);
    let me_pub_ip = public_ip::addr().await;
    info!("server pulic ip: {}, private ip: {}",&conn.server.pub_ip, &conn.server.pri_ip );
    if me_pub_ip.is_some() {
        info!("current public ip: {}",me_pub_ip.unwrap().to_string())
    };
        
    if conn.jwt.is_none(){
        debug!("no jwt will try to login ");
        let name  = conn.user.usrname;
        let cpid = conn.user.cpid;
        let paswd = conn.user.paswd;
        let req = LoginReq {
            cpid: cpid.clone(),
            name: name.clone(),
            paswd: paswd.clone(),
        };
        debug!("login request name:{}, cpid:{}, password:{} ;",&name, &cpid, &paswd);
        let request = req.sz().unwrap();
        let private_s =  TcpStream::connect(&pri_addr).await;
        let mut stream = if private_s.is_ok() {
            info!("trying private ip: {:?}",&pri_addr);
            private_s.unwrap()
        } else {
            debug!("faild to connect to private");
            info!("trying public ip: {:?}", &pub_addr);
            TcpStream::connect(pub_addr)
                .await
                .expect("could not connect to public ip")
        };
        let is_who_server = handshakes::client(
            &mut stream,
            &conn.server,
            pool
        ).await?;
        if is_who_server != 0 {
            exit(1);
        };
        
        stream.write_u8(LOGIN_CRED).await?;

        stream.flush().await?;
        let reques_len = request.len();
        debug!("login request size: {:?}", reques_len);
        let size = stream.write(&request).await?;
        stream.flush().await?;
        debug!("request was sent: bytes sent {:?}",size);
        assert_eq!(size, reques_len);
        debug_assert_eq!(size, reques_len);
        drop(request);

        let what = stream.read_u8().await?;
        debug!("SERVER: {}",what);
        if what == 0 {
            let mut jwt_buf = vec![0;500];
            let size = stream.read(&mut jwt_buf).await.unwrap();
            let jwt = String::from_utf8(jwt_buf[..size].to_vec()).unwrap();
            add_jwt(&conn.server.cpid, &jwt, &cpid, pool).await;
            stream.write_u8(0).await?;
            drop(stream);
            return Ok(8)
        }
        else {
            match what {
                UNAUTHORIZED => {
                    error!(
                    "SERVER: you are not authorized to log in
                        check if you are an already a in the db used by the server 
                        will exit not with status of 1"
                    );
                    exit(1)
                    
                } 
                _ => {Ok(44)}
            }
        }
    } else  {
        debug!("Will use jwt auth");
        let jwt = conn.jwt.unwrap();
        let req = JwtReq {
            jwt,
            request: rqm.clone()
        };
        let extra_jwt = req.jwt.clone();
        let request = req.sz().unwrap();

        let private_s =  TcpStream::connect(&pri_addr).await;
        let mut stream = if private_s.is_ok() {
            info!("trying private ip: {:?}",&pri_addr);

            info!("Connected to  private ip: {:?}", &pri_addr);
            private_s.unwrap()
        } else {
            debug!("faild to connect to private");
            info!("trying public ip: {:?}", &pub_addr);
            TcpStream::connect(pub_addr)
                .await
                .expect("could not connect to public ip")
        };
        let is_who_server = handshakes::client(
            &mut stream,
            &conn.server,
            pool
        ).await?;
        if is_who_server != 0 {
            exit(1);
        };
         
        stream.write_u8(JWT_AUTH).await?;
        stream.flush().await?;
        debug!("sent auth state {}",JWT_AUTH);
        let size = stream.write(&request).await?;
        stream.flush().await?;
        let is_valid = stream.read_u8().await?;
        if is_valid != 0 {
            delete_jwt(&extra_jwt, pool).await.unwrap();
            info!("there was an unexpected jwt change please run the same command again");
            exit(UNAUTHORIZED as i32)
        };
        info!("sent full request with size: {:?}",size);
        let state = handle_client_request(
            &mut stream,
            rqm,
            conn.server.cpid,
            check_for_sum,
            pool
        ).await.unwrap();
        if state == 0 {
            info!("request request was succesful");
        }
        else {
            warn!("a request was unsuccesful");
            println!("a request was unsuccesful");
            return Ok(1);
        }
        
        Ok(0)
    }
}
