use std::net::{IpAddr, SocketAddr};
use std::io;
use common_lib::cheat_sheet::TCP_MAIN_PORT;
use lib_db::types::PgPool;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use crate::client::client::Cred;


pub async fn connect_tcp(ip: IpAddr, pool: &PgPool, cred: Option<Cred> , jwt: Option<String>) -> io::Result<()> {
    let port = TCP_MAIN_PORT;
    let addr = SocketAddr::new(ip, port);
    let mut stream = TcpStream::connect(addr).await?;
    stream.write_all(b"a fast_res").await?;
    Ok(())
}
