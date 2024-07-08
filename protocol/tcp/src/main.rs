use std::net::{TcpListener,TcpStream};
//use std::net::SocketAddr;
//use tokio::io::Error;
fn main() {
    //let socket_addr: &SocketAddr = &self.address.parse().unwrap();
    let listener = TcpListener::bind("0.0.0.0:1986");
    println!("listining on 0000:1986");

}

/*
will work on the sessions later and and finish the tcp protocol in a week give or take then init the main server that will use 
Tcp 

*/