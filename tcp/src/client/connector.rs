use std::net::SocketAddr;
use std::io;
use common_lib::cheat_sheet::TCP_MAIN_PORT;
use lib_db::types::PgPool;
use lib_db::user::user_jwts::add_jwt;
use log::warn;
use serde::Serialize;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::client::client::Connection;
use crate::common::request::{JwtReq, LoginReq, JWT_AUTH, RQM};
use super::handle_request::handle_client_request;




pub async fn connect_tcp(pool: &PgPool, conn: Connection, type_:  , request: RQM) -> io::Result<u8> {
    if conn.jwt.is_none() && conn.cred.is_some() {
        let mut jwt: String = String::new();
        let port = TCP_MAIN_PORT;
        let addr = SocketAddr::new(conn.ip, port);
        let mut stream = TcpStream::connect(addr).await?;
        let cred = conn.cred.unwrap();
        let name = cred.name;
        let cpid = cred.cpid;
        let paswd = cred.paswd;
        let req = LoginReq {
            cpid,
            name,
            paswd
        };
        let request = req.sz().unwrap();
        stream.write_all(&request).await?;
        stream.flush().await?;
        
        stream.read_to_string(&mut jwt).await?;
        add_jwt(jwt, conn.host, pool).await.unwrap();
        drop(stream);
        return Ok(8)
    }
    
    let jwt = conn.jwt.unwrap();
    let req = JwtReq {
        jwt,
        request_type:  type_,
        request
    };
    let request = req.sz().unwrap();

    let port = TCP_MAIN_PORT;
    let addr = SocketAddr::new(conn.ip, port);
    let mut stream = TcpStream::connect(addr).await?;

    stream.write_u8(JWT_AUTH).await?;
    stream.flush().await?;

    stream.write_all(&request).await?;
    stream.flush().await?;

    let state = 0;
    if state == 0 {
        println!("request request was succesful");
    }
    else {
        warn!("a request was unsuccesful");
        return Ok(1);
    }
    

    
    Ok(0)
}
