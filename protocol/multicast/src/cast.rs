use std::net::{IpAddr, SocketAddr};

use tokio::net::UdpSocket; 
use common_lib::cheat_sheet::MULTICAST_PORT;

// msg form : 0 or 1// server or client name // ip // cpid #


pub async fn cast_and_buffer(ip: IpAddr, command: u8) 
    {
    let addr = SocketAddr::new(ip, MULTICAST_PORT);
    let socket = UdpSocket::bind(addr).await.unwrap();
    if command == 0 
    {
        let msg = b"hello world is any body there\n";
        socket.send(msg).await.unwrap();
        // TODO make the buffer smaller
        let mut buffer1 = Vec::new();
        let (size, src) = socket.recv_from(&mut buffer1).await.expect("could not get buffer");
        println!("source: {}", src);
        let str_answer = String::from_utf8_lossy(&buffer1[..size]);
        println!("{}",str_answer);
    }
    if command == 1 
    {
        loop 
        {
            let mut buffer = Vec::new();
            let (size, src) = socket.recv_from(&mut buffer).await.unwrap();
            if size > 0 
            {
                let string_msg = String::from_utf8_lossy(&buffer[..size]);
                let message_info :Vec<&str> = string_msg.split("//").collect();
               



            } 
        }
    }

}




