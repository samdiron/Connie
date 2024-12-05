use tokio;
use std::net::{IpAddr, SocketAddr,TcpListener};
use common_lib::cheat_sheet::{LOCAL_IP,TCP_MAIN_PORT};


pub fn tcp_listener() {
    let ip: String = LOCAL_IP.clone();
    let port: u16 = TCP_MAIN_PORT.clone();
    let ip = ip.parse::<IpAddr>().expect("could not parse ip from string"); 
    let socket_addr = SocketAddr::new(ip, port);
    let listener = TcpListener::bind(socket_addr)
        .expect("could not bind tcp socket on port 4443 ");
    match listener.accept() {
        Ok((_socket , addr)) => {
            println!("a new client: {:?}",addr);

        }
        Err(e) =>{
            eprintln!("{}",e)
        }

    }
}
