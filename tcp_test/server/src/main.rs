#![warn(unused_variables)]
//#![warn(unused_imports)]

use std::io;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

const STATE: u8 = 1;

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    //let state: u8 = 0;
    //let buff = 1;

    stream.write(&STATE.to_be_bytes());
    let _ = stream.flush();
    //let msg = stream.read(&mut [0; 3])?;
    //println!("client: {}", &msg);
    println!("passed to handle_connection");
    Ok(())
}

fn main() {
    let socket = TcpListener::bind("0.0.0.0:1909").expect("fail to bind");
    println!("connected to port");
    for stream in socket.incoming() {
        match stream {
            Ok(stream) => {
                let _ = handle_connection(stream);
                println!("a client connected");
            }
            Err(e) => {
                eprintln!("Error in match : {}", e);
            }
        }
    }
}
