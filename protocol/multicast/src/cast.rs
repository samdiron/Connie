use std::{
    net::UdpSocket,
    net::{IpAddr, SocketAddr},
};

use common_lib::cheat_sheet::MULTICAST_PORT;
// use tokio::net::UdpSocket;

// TODO : make athis function
// async fn validate_multicast() {
// used to validate the multicast an then do a handshake and to look up
// if the unit is a connie and if it's already connected before
// example of handshake req {
//      local/public :0/1 ?.
//      status: 0/1 ?.
//      srerver_name or client_name ?.
//      cpid ?.
//      (ready to connect status: 0/1) ?.
//      (connie Version) .
//
//
// }
// note ?. is used as a split psattern due to being not a common
// pattern for names or password
// or cpid
// and at the end a single . to mark the end of the handshake req
//
// }

//TODO :create a function to make the handshake file

// msg form : 0 or 1// server or client name // cpid #
// to save power we could replace the ip from the msg by using recv_from src ;
pub async fn cast_and_buffer(ip: IpAddr, command: u8) {
    let addr = SocketAddr::new(ip, MULTICAST_PORT);
    let socket = UdpSocket::bind(addr).unwrap();
    if command == 0 {
        let msg = b"hello world is any body there\n";
        socket.send(msg).unwrap();
        // TODO make the buffer smaller
        let mut buffer1 = Vec::new();
        let (size, src) = socket
            .recv_from(&mut buffer1)
            .expect("could not get buffer");
        println!("source: {}", src);
        let str_answer = String::from_utf8_lossy(&buffer1[..size]);
        println!("{}", str_answer);
    }
    if command == 1 {
        loop {
            let mut buffer = vec![0; 200];
            let (size, src) = socket.recv_from(&mut buffer).unwrap();
            if size > 0 {
                let string_msg = String::from_utf8_lossy(&buffer[..size]);
                let string_msg = string_msg.into_owned();
                let mut message_info: Vec<&str> = string_msg.split("//").collect();
                message_info.push(src.to_string().as_str());
            }
        }
    }
}
