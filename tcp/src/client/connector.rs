use std::net::SocketAddr;
use std::io;
use common_lib::cheat_sheet::TCP_MAIN_PORT;
use lib_db::types::PgPool;
use lib_db::user::user_jwts::add_jwt;
use log::warn;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::client::client::Connection;
use crate::common::request::{format_jwt_request, format_login_request};
use super::handle_request::handle_client_request;




pub async fn connect_tcp(pool: &PgPool, conn: Connection, raw_request: String) -> io::Result<u8> {
    if conn.jwt.is_none() {
        let mut jwt: String = String::new();
        let port = TCP_MAIN_PORT;
        let addr = SocketAddr::new(conn.ip, port);
        let mut stream = TcpStream::connect(addr).await?;
        let cred = conn.cred.unwrap();
        let cred_vec = vec![cred.cpid.as_str(), cred.paswd.as_str()];
        let request = format_login_request(cred_vec);
        stream.write_all(b"a fast_res").await?;
        stream.flush().await?;

        stream.write_all(request.as_bytes()).await?;
        stream.flush().await?;
        
        stream.read_to_string(&mut jwt).await?;
        add_jwt(jwt, conn.host, pool).await.unwrap();
        drop(stream);
        return Ok(8)
    }
    
    let jwt = conn.jwt.unwrap();
    let _request = format_jwt_request(jwt, raw_request.clone());
    let port = TCP_MAIN_PORT;
    let addr = SocketAddr::new(conn.ip, port);
    let mut stream = TcpStream::connect(addr).await?;
    stream.write_all(b"a fast_res").await?;
    stream.flush().await?;

    stream.write_all(&_request.as_bytes()).await?;
    stream.flush().await?;

    let state = handle_client_request(&mut stream, raw_request).await?;
    if state == 0 {
        println!("request request was succesful");
        
    }
    else {
        warn!("a request was unsuccesful");
        return Ok(1);
    }
    

    
    Ok(0)
}
