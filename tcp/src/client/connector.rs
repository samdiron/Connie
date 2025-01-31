use std::net::{IpAddr, SocketAddr};
use std::io;
use common_lib::cheat_sheet::TCP_MAIN_PORT;
use lib_db::types::PgPool;
use lib_db::user::user_jwts::add_jwt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::client::client::Connection;
use crate::common::request::format_login_request;


// async fn server_request(request: String, stream: &mut TcpStream) {
//
// }


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

        return Ok(8)
    
    }    
    let port = TCP_MAIN_PORT;
    let addr = SocketAddr::new(conn.ip, port);
    let mut stream = TcpStream::connect(addr).await?;
    stream.write_all(b"a fast_res").await?;
    Ok(0)
}
