use std::net::{IpAddr, SocketAddr};

use std::net::UdpSocket; 
use common_lib::cheat_sheet::MULTICAST_PORT;


pub fn cast_and_buffer(ip: IpAddr) {
    let addr = SocketAddr::new(ip, MULTICAST_PORT);
    let socket = UdpSocket::bind(addr).unwrap();
    let msg = b"hello world is any body there\n";
    socket.send(msg).unwrap();
    let mut buffer = [0; 30];
    let answer = socket.recv(&mut buffer).expect("could not get buffer");
    let str_answer= answer.to_string();
    println!("{:?}",str_answer);

}  


