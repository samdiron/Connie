use std::io;
use std::io::Write;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;

pub fn tcp_connector(ip: &str) -> io::Result<()> {
    let ip = IpAddr::from_str(ip).unwrap();
    let port = common_lib::cheat_sheet::TCP_MAIN_PORT;
    let socketaddr = SocketAddr::new(ip, port);
    let mut stream = TcpStream::connect(socketaddr).unwrap();
    let text = b"text from a client ?.";
    let _res = stream.write_all(text).unwrap();

    Ok(())
}
