use common_lib::cheat_sheet::LOCAL_IP;
use common_lib::cheat_sheet::SYSTEM_TCP;
use log::{info, trace, warn};
use std::io::Read;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;

fn handle_conn(stream: (TcpStream, SocketAddr)) {
    let addr = stream.1;
    let ip = LOCAL_IP.clone();
    let mut stream = stream.0;
    if addr.ip() != ip {
        let _ = stream.shutdown(std::net::Shutdown::Both);
    }

    let mut buffer = vec![0; 120];
    let size = stream.read_to_end(&mut buffer).unwrap();
    println!("bytes read : {:?}", &buffer[..size]);
    // should compare the message and exute the corosponenig command
}

pub fn process() -> std::io::Result<()> {
    trace!("started the control socket");
    let ip = LOCAL_IP.clone();
    let port = SYSTEM_TCP;
    let sock_addr = SocketAddr::new(ip, port);
    let socket = TcpListener::bind(sock_addr).expect("could not bind system tcp socket");
    match socket.accept() {
        Ok(stream) => {
            info!("incoming control socket");
            handle_conn(stream);
        }
        Err(err) => {
            warn!("control socket: {}", err);
        }
    }
    Ok(())
}
