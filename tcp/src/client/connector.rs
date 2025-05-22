
use core::convert::From;
use std::thread;
use std::io::Result;
use std::process::exit;
use std::time::Duration;
use std::net::{IpAddr, SocketAddr};

use lib_db::types::SqlitePool;
use lib_db::sqlite::sqlite_host::SqliteHost;
use lib_db::sqlite::sqlite_user::{ShortUser, SqliteUser};
use lib_db::sqlite::sqlite_jwt::{
    add_jwt, delete_jwt,
    delete_user_jwt
};

use common_lib::public_ip;
use common_lib::tokio::time::timeout;
use common_lib::tokio::net::TcpStream;
use common_lib::log::{debug, info, warn, error};
use common_lib::tokio::io::{
    AsyncReadExt,
    AsyncWriteExt
};

use tokio_rustls::client;
use tokio_rustls::TlsConnector;
use tokio_rustls::rustls::pki_types::ServerName;

use crate::types::LOGIN_CRED;
use crate::common::handshakes;
use crate::client::config::make_config;
use crate::client::client::Connection;
use crate::common::util::client::{rvfs, wvts};
use crate::client::handle_request::{
    handle_client_request,
    raw_handle_client_request
};
use crate::common::request::req_format::{JwtReq, LoginReq};
use crate::common::request::{
    JWT_AUTH, RQM, SERVER_WILL_NOT_ALLOW_NOTLS, SIGNIN_CRED, UNAUTHORIZED
};



pub async fn signup_process(
    addr: SocketAddr,
    user: ShortUser,
    pool: &SqlitePool
) -> Result<()> {
    let short_user_vec = user.sz().unwrap();
    let mut stream = TcpStream::connect(&addr).await?;
    info!("connected to {addr}");

    // write connection status 
    stream.write_u8(0).await?; 


    let server_name = get_server_name(addr.ip());
    let mut stream = get_tlstream(stream, server_name)
        .await?;
    info!("Connected tls");
    let d: Vec<u8> = vec![SIGNIN_CRED];
    wvts(Some(&mut stream), None, d).await?;
    debug!("sent status to host {SIGNIN_CRED}");
    let will_allow = stream.read_u8().await?;
    if will_allow == 0 {
        info!("server accsepted request");
        let vector = rvfs(Some(&mut stream), None).await?;
        let server = SqliteHost::dz(vector).unwrap();
        info!("server: name: {}, host: {};",&server.name, &server.host);
        info!("the thread will pause for 2s if you want to cancel type ^c");
        let dur = Duration::from_secs(2);
        thread::sleep(dur);
        stream.write_u8(0).await?;
        stream.flush().await?;
        wvts(Some(&mut stream), None, short_user_vec).await?;

        let user_vec = rvfs(Some(&mut stream), None).await?;
        
        let mut user = SqliteUser::dz(user_vec).unwrap();
        user.host = server.cpid.clone();
        debug!("host name: {}",&user.host);
        SqliteUser::add_user(user, pool).await.unwrap();
        SqliteHost::new(server, pool).await;

        let status = stream.read_u8().await?;
        assert_eq!(status, 0);
        stream.shutdown().await?;
    }
    else {
        info!("server did not allow to signup");
    }

    Ok(())
    
}

#[allow(dead_code)]
pub(in crate::client) async fn get_tlstream(
    raw_stream: TcpStream,
    server_name: ServerName<'static>,
) -> Result<client::TlsStream<TcpStream>>{
    let config = make_config();
    debug!("config: succesful");
    let connector = TlsConnector::from(config);
    debug!("connector: succesful");
    let stream = connector.connect(server_name, raw_stream).await;
    if stream.is_err() {
        debug!("tls stream: not succesful");
        let e = stream.unwrap_err();
        eprintln!("{:?}", e.to_string());
        error!("will exit with status 1");
        exit(1);
    };
    Ok(stream.unwrap())
}


async fn get_stream(
    host: &SqliteHost,
    port: Option<u16>,
    ip: Option<IpAddr>,
) -> Result<(TcpStream, IpAddr)> {
    let port = if port.is_some(){port.unwrap()} else {host.port};
    
    let pub_ip: IpAddr = host.pub_ip.parse().unwrap();
    let pri_ip: IpAddr = host.pri_ip.parse().unwrap();
    
    let pub_addr = SocketAddr::new(pub_ip, port);
    let pri_addr = SocketAddr::new(pri_ip, port);
    let addr: IpAddr;

    let dur = Duration::from_secs_f32(0.20);
    let me_pub_ip_bind = timeout(dur, public_ip::addr()).await;
    let me_pub_ip = if me_pub_ip_bind.is_ok() {
        me_pub_ip_bind.unwrap()
    } else {None};
    info!(
        "server pulic ip: {}, private ip: {}",
        &host.pub_ip,
        &host.pri_ip 
    );
    if me_pub_ip.is_some() {
        info!("current public ip: {}",me_pub_ip.unwrap().to_string())
    };
    if ip.is_none() {
    let dur = Duration::from_secs_f32(0.40);
    let pri_s = timeout(dur, TcpStream::connect(pri_addr)).await;
    let stream = if pri_s.is_ok() {
        let pri_s = pri_s.unwrap();
        info!("trying private ip: {:?}",&pri_addr);
        addr = pri_ip;
        pri_s.unwrap()

    } else {
        debug!("faild to connect to private");
        info!("trying public ip: {:?}", &pub_addr);
        addr = pub_ip;
        TcpStream::connect(&pub_addr)
            .await
            .expect("could not connect to public ip")

    };
    
    Ok((stream, addr))
    } else {
        let ip = ip.unwrap();
        let addr = ip.clone();
        let socket_addr = SocketAddr::new(ip, port);
        debug!("will use custom ip: {}", &socket_addr);
        let stream = TcpStream::connect(&socket_addr)
            .await
            .expect("could not connect to custom ip");
        Ok((stream, addr))
    }
} 



fn get_server_name(addr: IpAddr) -> ServerName<'static> {
    ServerName::from(addr)
}


pub async fn login_request(
    pool: &SqlitePool,
    conn: Connection,
    passwd: String,
    notls: bool,
) -> Result<u8> {
    assert!(conn.jwt.is_none());
    info!("deleting old jwts");

    delete_user_jwt(
        pool,
        &conn.user.cpid,
        &conn.server.cpid
    ).await;
    let name  = conn.user.usrname.clone();
    let cpid = conn.user.cpid.clone();
    let req = LoginReq {
        cpid: cpid.clone(),
        name: name.clone(),
        paswd: passwd,
    };
    let request = req.sz().unwrap();
    let (mut stream, ip_addr) = get_stream(
        &conn.server,
        conn.port,
        conn.ip
    ).await?;

    if notls {
        stream.write_u8(1).await?;
        let server_will_allow_no_tls = stream.read_u8().await?;
        if server_will_allow_no_tls == SERVER_WILL_NOT_ALLOW_NOTLS {
            error!(
                "SERVER will not allow notls connections with status: {}",
                SERVER_WILL_NOT_ALLOW_NOTLS 
            );
            exit(SERVER_WILL_NOT_ALLOW_NOTLS as  i32);
        };
        let res = notls_login_helper(
            stream,
            request,
            pool,
            conn,
        ).await?;
        return Ok(res);
    };

    // write the connection status no_tls == 1 / tls == 0
    stream.write_u8(0).await?;

    let tls_server_name = get_server_name(ip_addr);
    let mut stream = get_tlstream(stream, tls_server_name)
        .await
        .expect("could not connect tls");
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

    let confirm = stream.read_u8().await?;
    debug!("SERVER: {}",confirm);
    if confirm == 0 {
        let mut jwt_buf = vec![0;500];
        let size = stream.read(&mut jwt_buf).await.unwrap();
        let jwt = String::from_utf8(
            jwt_buf[..size]
                .to_vec()
        ).unwrap();
        add_jwt(&conn.server.cpid, &jwt, &cpid, pool).await;
        stream.write_u8(0).await?;
        drop(stream);
        return Ok(0)
    }
    else {
        match confirm {
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
}

async fn notls_login_helper(
    mut stream: TcpStream,
    request: Vec<u8>,
    pool: &SqlitePool,
    conn: Connection,
) -> Result<u8> {
    let cpid = conn.user.cpid;
    let is_who_server = handshakes::raw_client_handshake(
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

    let confirm = stream.read_u8().await?;
    debug!("SERVER: {}",confirm);
    if confirm == 0 {
        let mut jwt_buf = vec![0;500];
        let size = stream.read(&mut jwt_buf).await.unwrap();
        let jwt = String::from_utf8(
            jwt_buf[..size]
                .to_vec()
        ).unwrap();
        add_jwt(&conn.server.cpid, &jwt, &cpid, pool).await;
        stream.write_u8(0).await?;
        drop(stream);
        return Ok(0)
    }
    else {
        match confirm {
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
}





pub async fn connect_tcp(
    pool: &SqlitePool,
    conn: Connection,
    check_for_sum: Option<bool>,
    rqm: Option<RQM>,
    no_tls: bool,
) -> Result<u8> {
    debug!("Will use jwt auth");
    assert!(rqm.is_some());
    assert!(conn.jwt.is_some());

    let rqm = rqm.unwrap();
    let jwt = conn.jwt.unwrap();
    let req = JwtReq {
        jwt,
        request: rqm.clone()
    };
    let extra_jwt = req.jwt.clone();
    let request = req.sz().unwrap();

    let (
        mut stream,
        ip_addr
    ) = get_stream(&conn.server, conn.port, conn.ip).await?;
    if no_tls {
        info!("will not be using tls");
        stream.write_u8(1).await?;

        let server_will_allow_no_tls = stream.read_u8().await?;
        if server_will_allow_no_tls == SERVER_WILL_NOT_ALLOW_NOTLS {
            error!(
                "SERVER will not allow notls connections with status: {}",
                SERVER_WILL_NOT_ALLOW_NOTLS 
            );
            exit(SERVER_WILL_NOT_ALLOW_NOTLS as  i32);
        };
            
        let is_who_server = handshakes::raw_client_handshake(
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
            info!("there was an unexpected jwt change please run the login request flag(-l or --login ) with true ");
            exit(UNAUTHORIZED as i32)
        };

        debug!("sent full request with size: {:?}",size);
        let state = raw_handle_client_request(
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

    } else {
        stream.write_u8(0).await?;
        let tls_server_name = get_server_name(ip_addr);
        let mut stream = get_tlstream(stream, tls_server_name).await?;
            
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
            info!("there was an unexpected jwt change please run the login request flag(-l or --login ) with true ");
            exit(UNAUTHORIZED as i32)
        };

        debug!("sent full request with size: {:?}",size);
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
