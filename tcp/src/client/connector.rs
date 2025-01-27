use std::net::{IpAddr, SocketAddr};
use std::io;
use common_lib::cheat_sheet::TCP_MAIN_PORT;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub async fn connect_tcp(ip: IpAddr, jwt: String) -> io::Result<()> {
    let port = TCP_MAIN_PORT;
    let addr = SocketAddr::new(ip, port);
    let mut stream = TcpStream::connect(addr).await?;
    stream.write(&jwt.as_bytes()).await?;

    Ok(())
}
