use std::io::Result;
use std::net::SocketAddr;
use std::time::Duration;
use std::thread;
use std::process::exit;
use common_lib::cheat_sheet::{PUB_IP, TCP_MAIN_PORT};
use lib_db::sqlite::sqlite_host::SqliteHost;
use lib_db::sqlite::sqlite_user::{ShortUser, SqliteUser};
use lib_db::types::{PgPool, SqlitePool};
use lib_db::sqlite::sqlite_jwt::{add_jwt, delete_jwt};
use common_lib::log::{debug, info, warn, error};
use common_lib::tokio::io::{
    AsyncReadExt,
    AsyncWriteExt
};
use common_lib::tokio::net::TcpStream;
use crate::client::client::Connection;
use crate::common::request::{
    JWT_AUTH, RQM, SIGNIN_CRED, UNAUTHORIZED
};
use crate::common::request::req_format::{JwtReq, LoginReq};
use crate::common::util::{rvfs, wvts};
use crate::types::LOGIN_CRED;
use super::handle_request::handle_client_request;



pub async fn signup_process(addr: SocketAddr, user: ShortUser, pool: &SqlitePool) -> Result<()> {
    let short_user_vec = user.sz().unwrap();
    let mut stream = TcpStream::connect(addr).await?;
    stream.write_u8(SIGNIN_CRED).await?;
    stream.flush().await?;
    
    let will_allow = stream.read_u8().await?;
    if will_allow == 0 {
        let vector = rvfs(&mut stream).await?;
        let server = SqliteHost::dz(vector).unwrap();
        info!("server: name: {}, host: {};",&server.name, &server.host);
        info!("the thread will pause for 2s if you want to cancel type ^c");
        let dur = Duration::from_secs(2);
        thread::sleep(dur);
        stream.write_u8(0).await?;
        wvts(&mut stream, short_user_vec).await?;

        let user_vec = rvfs(&mut stream).await?;
        
        let user = SqliteUser::dz(user_vec).unwrap();
        SqliteUser::add_user(user, pool).await.unwrap();
        SqliteHost::new(server, pool).await;

    }

    Ok(())
    
}


pub async fn connect_tcp(pool: &SqlitePool, conn: Connection, rqm: RQM) -> Result<u8> {
    if conn.jwt.is_none(){
        debug!("CLIENT: no jwt will try to login ");
        let port = conn.server.port;
        let addr = if *PUB_IP.to_string() != conn.server.pub_ip {
            SocketAddr::new(conn.server.pub_ip.parse().unwrap(), port)
        } else {
            SocketAddr::new(conn.server.pri_ip.parse().unwrap(), port) 
        };
        // debug!("sent to host: cpid: {}, paswd: *********",&cpi);
        let req = LoginReq {
            cpid: conn.user.cpid.clone(),
            name: conn.user.name.clone(),
            paswd: conn.user.paswd.clone(),
        };
        let request = req.sz().unwrap();
        let mut stream = TcpStream::connect(&addr).await?;
        info!("connected to host: {:?}",addr);

        stream.write_u8(LOGIN_CRED).await?;
        stream.flush().await?;
        let reques_len = request.len();
        debug!("CLIENT: login request size: {:?}", reques_len);
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
            add_jwt(&conn.server.cpid, &jwt, &conn.user.cpid.clone(), pool).await;
            stream.write_u8(0).await?;
            drop(stream);
            return Ok(8)
        }
        else {
            match what {
                UNAUTHORIZED => {
                    error!("SERVER: you are not authorized to log in\n check if you are an already a in the db used by the server \n will exit not with status of 1");
                    exit(1)
                    
                } 
                _ => {Ok(44)}
            }
        }
    } else  {
        println!("CLIENT: Will use jwt auth");
        let jwt = conn.jwt.unwrap();
        let req = JwtReq {
            jwt,
            request: rqm.clone()
        };
        let extra_jwt = req.jwt.clone();
        let request = req.sz().unwrap();

        let port = conn.server.port;
        let addr = if *PUB_IP.to_string() != conn.server.pub_ip {
            SocketAddr::new(conn.server.pub_ip.parse().unwrap(), port)
        } else {
            SocketAddr::new(conn.server.pri_ip.parse().unwrap(), port) 
        };
        
        let mut stream = TcpStream::connect(&addr).await?;
        info!("CLIENT: connected to {}",&addr);
        stream.write_u8(JWT_AUTH).await?;
        stream.flush().await?;
        debug!("CLIENT: sent auth state {}",JWT_AUTH);
        let size = stream.write(&request).await?;
        stream.flush().await?;
        let is_valid = stream.read_u8().await?;
        if is_valid != 0 {
            delete_jwt(&extra_jwt, pool).await.unwrap();
            info!("there was an unexpected jwt change please run the same command again");
            exit(UNAUTHORIZED as i32)
        };
        info!("CLIENT: sent full request with size: {:?}",size);
        let state = handle_client_request(&mut stream, rqm).await.unwrap();
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
