#![warn(unused_imports)]
use std::io;
use std::io::Read;
use std::net::{Shutdown, TcpListener, TcpStream}; //, prelude::*};
                                                  //mod sgin_in;
                                                  //use surreal::User;
                                                  //mod User;
                                                  //use tokio::net::{TcpListener, TcpStream};
use tokio;
#[tokio::sync]
async fn handle_conn(mut stream: TcpStream) -> io::Result<()> {
    let mut buff = [0; 80];
    let stream_bytes = stream.read(&mut buff)?;
    let msg = String::from_utf8_lossy(&buff[..stream_bytes]);
    if msg.len() > 80 {
        let _ = stream.shutdown(Shutdown::Both);
    } else {
        //shadowing the msg_data to do deffirent for the futur it will be made into a struct that
        //have more layers of  protection from sql injections
        let msg_data = msg.to_lowercase();
        let msg_data: &str = msg_data.as_str();
        println!("stream: {}", msg);
        let data = ["abc.123", "else"];
        if data.contains(&msg_data) {
            println!("a user: {} ", msg_data);
            let user: User = User {
                cpid: String::from("dindinlk.1"),
                pass: String::from("dindinlk.1"),
            };
            let _ = user.sgin_in();
        } else {
            println!("not a user: {}", msg_data);
        }
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
