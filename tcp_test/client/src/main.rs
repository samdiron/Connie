#![warn(unused_variables)]
//#![warn(unused_imports)]
use std::io;
use std::io::prelude::*;
//use std::io::{Read, Write};
//use std::net::TcpListener;
use std::net::TcpStream;

const STATE: u8 = 1;

fn main() -> io::Result<()> {
    let mut server = TcpStream::connect("0.0.0.0:1909").expect("fail to connect");
    println!("connected to port");
    let msg = server.read(&mut [1])?;
    println!("server read");
    //server.write(&mut [2])?;
    Ok(())
}
