use std::net::{IpAddr, SocketAddr};

use tokio::net::UdpSocket; 
use common_lib::cheat_sheet::MULTICAST_PORT;


pub async fn cast_and_buffer(ip: IpAddr) {
    let addr = SocketAddr::new(ip, MULTICAST_PORT);
    let socket = UdpSocket::bind(addr).await.unwrap();
    let msg = b"hello world is any body there";
    socket.send(msg).await.unwrap();

}  


