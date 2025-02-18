use std::net::SocketAddr;
use std::io;
use std::process::exit;
use common_lib::cheat_sheet::TCP_MAIN_PORT;
use lib_db::types::PgPool;
use lib_db::user::user_jwts::add_jwt;
use log::{debug, info, warn};
use tokio::io::{
    AsyncReadExt,
    AsyncWriteExt
};
use tokio::net::TcpStream;
use crate::client::client::Connection;
use crate::common::request::{
    JWT_AUTH,
    RQM,
    UNAUTHORIZED
};
use crate::common::request::req_format::{JwtReq, LoginReq};
use crate::types::LOGIN_CRED;
use super::handle_request::handle_client_request;




pub async fn connect_tcp(pool: &PgPool, conn: Connection, rqm: RQM) -> io::Result<u8> {
    if conn.jwt.is_none(){
        println!("CLIENT: no jwt will try to login ");
        let port = TCP_MAIN_PORT;
        let addr = SocketAddr::new(conn.ip, port);
        let cred = conn.cred;
        let name = cred.name;
        let cpid = cred.cpid;
        let paswd = cred.paswd;
        println!("sent to host: cpid: {}, paswd: *********",&cpid);
        let req = LoginReq {
            cpid: cpid.clone(),
            name,
            paswd
        };
        let request = req.sz().unwrap();
        let mut stream = TcpStream::connect(&addr).await?;
        println!("connected to host: {:?}",addr);

        stream.write_u8(LOGIN_CRED).await?;
        stream.flush().await?;
        let reques_len = request.len();
        println!("CLIENT: login reqeust size: {:?}", reques_len);
        debug!("CLIENT: login request size: {:?}", reques_len);
        let size = stream.write(&request).await?;
        stream.flush().await?;
        println!("request was sent: bytes sent {:?}",size);
        assert_eq!(size, reques_len);
        debug_assert_eq!(size, reques_len);
        drop(request);

        let what = stream.read_u8().await?;
        println!("SERVER: {}",what);
        if what == 0 {
            let mut jwt_buf = vec![0;500];
            let size = stream.read(&mut jwt_buf).await.unwrap();
            let jwt = String::from_utf8(jwt_buf[..size].to_vec()).unwrap();
            add_jwt(jwt, conn.host, cpid, pool).await.unwrap();
            stream.write_u8(0).await?;
            drop(stream);
            return Ok(8)
        }
        else {
            match what {
                UNAUTHORIZED => {
                    println!("SERVER: you are not authorized to log in\n check if you are an already a in the db used by the server \n will exit not with status of 1");
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
        let request = req.sz().unwrap();

        let port = TCP_MAIN_PORT;
        let addr = SocketAddr::new(conn.ip, port);
        let mut stream = TcpStream::connect(&addr).await?;
        println!("CLIENT: connected to {}",&addr);
        debug!("CLIENT: connected to {}",&addr);
        info!("CLIENT: connected to {}",&addr);
        stream.write_u8(JWT_AUTH).await?;
        stream.flush().await?;
        debug!("CLIENT: sent auth state {}",JWT_AUTH);
        println!("CLIENT: sent auth state {}",JWT_AUTH);
        

        let size = stream.write(&request).await?;
        stream.flush().await?;
        println!("CLIENT: sent full request with size: {:?}",size);
        let state = handle_client_request(&mut stream, rqm).await.unwrap();
        if state == 0 {
            println!("request request was succesful");
        }
        else {
            warn!("a request was unsuccesful");
            return Ok(1);
        }
        
        
        
        Ok(0)
    }
}
