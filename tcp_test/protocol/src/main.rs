#![warn(unused_imports)]
use std::io::Read;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::{io, prelude::*};

//use tokio::net::{TcpListener, TcpStream};
//use tokio;

fn handle_conn(mut stream: TcpStream) -> io::Result<()> {
    let mut buff = [0; 80];
    let stream_bytes = stream.read(&mut buff)?;
    let msg = String::from_utf8_lossy(&buff[..stream_bytes]);
    if msg.len() > 80 {
        let _ = stream.shutdown(Shutdown::Both);
    } else {
        println!("stream: {}", msg);
    }
    Ok(())
}

fn main() {
    let socket: TcpListener = TcpListener::bind("0.0.0.0:9000").expect("fail to bing");
    println!("binded");
    for stream in socket.incoming() {
        match stream {
            Ok(stream) => {
                let _ = handle_conn(stream);
                println!("stream passed to handle");
            }
            Err(e) => {
                eprint!("{}", e);
            }
        }
    }
}
