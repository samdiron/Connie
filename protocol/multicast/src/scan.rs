use std::net::UdpSocket;





fn scan_net() {
    
    println!("hello world");
    let mut buffer: u8 ;
    let mut  socket = UdpSocket::bind("addr").expect("msg ");
    socket.connect("addr" ).expect("msg TODO");
}
