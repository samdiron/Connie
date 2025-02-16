use std::net::SocketAddr;
use std::io;
use common_lib::cheat_sheet::TCP_MAIN_PORT;
use lib_db::types::PgPool;
use lib_db::user::user_jwts::add_jwt;
use log::warn;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::client::client::Connection;
use crate::common::request::{JwtReq, LoginReq, JWT_AUTH, RQM};
use crate::common::util::read_stream;
use crate::types::LOGIN_CRED;
use super::handle_request::handle_client_request;




pub async fn connect_tcp(pool: &PgPool, conn: Connection, rqm: RQM) -> io::Result<u8> {
    if conn.jwt.is_none()  {
        let mut jwt: String = String::new();
        let port = TCP_MAIN_PORT;
        let addr = SocketAddr::new(conn.ip, port);
        let mut stream = TcpStream::connect(addr).await?;
        stream.write_u8(LOGIN_CRED).await?;
        stream.flush().await?;
        let cred = conn.cred;
        let name = cred.name;
        let cpid = cred.cpid;
        let paswd = cred.paswd;
        let req = LoginReq {
            cpid: cpid.clone(),
            name,
            paswd
        };
        let request = req.sz().unwrap();
        stream.write_all(&request).await?;
        stream.flush().await?;
        let jwt_buf = read_stream(&mut stream, 300).await?;
        let jwt = String::from_utf8(jwt_buf).unwrap();
        println!("jwt: {}", &jwt);
        stream.write_u8(0).await?;
        add_jwt(jwt, conn.host, cpid, pool).await.unwrap();
        drop(stream);
        return Ok(8)
    }
    
    let jwt = conn.jwt.unwrap();
    let req = JwtReq {
        jwt,
        request: rqm.clone()
    };
    let request = req.sz().unwrap();

    let port = TCP_MAIN_PORT;
    let addr = SocketAddr::new(conn.ip, port);
    let mut stream = TcpStream::connect(addr).await?;

    stream.write_u8(JWT_AUTH).await?;
    stream.flush().await?;

    stream.write_all(&request).await?;
    stream.flush().await?;
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
