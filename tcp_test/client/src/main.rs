#![warn(unused_variables)]
//#![warn(unused_imports)]
use std::io;
use std::io::prelude::*;
//use std::net::TcpListener;
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let mut server = TcpStream::connect("0.0.0.0:9000").expect("fail to connect");
    println!("connected to port");
    //let msg = server.read(&mut [1])?;
    //println!("server read {}", msg);
    let mut msg = "abc.123";
    let _ = server.write(&mut msg.as_bytes())?;
    let _ = server.flush();
    Ok(())
}
