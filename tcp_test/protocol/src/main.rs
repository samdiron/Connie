#![warn(unused_imports)]
use std::io;
use std::io::Read;
use std::net::{Shutdown, TcpListener, TcpStream};

use tokio;

use surreal_db::user::sgin_in;
use surreal_db::db::DB;

#[tokio::main]
async fn handle_conn(mut stream: TcpStream) -> io::Result<()> {
    let mut db = DB {
        addr: Option::from(String::from("0.0.0.0:8000")),
        name_sp: None,
        database: None,
        isremote: Option::from(false) ,
    };
    db.connect().await.expect("db could not connect");

    let mut buff = [0; 80];
    let stream_bytes = stream.read(&mut buff)?;
    let msg = String::from_utf8_lossy(&buff[..stream_bytes]);
    if msg.len() > 80 {
        let _ = stream.shutdown(Shutdown::Both);
    } else {
        //shadowing the msg_data to do different for the futureit will be made into a struct that
        //have more layers of  protection from sql injections
        let msg_data = msg.to_lowercase();
        let msg_data: &str = msg_data.as_str();
        println!("stream: {}", msg);
        let data = ["abc.123", "else"];
        if data.contains(&msg_data) {
            println!("a user: {} ", msg_data);
            let user: sgin_in::User = sgin_in::User {
                cpid: "dindinlk.1",
                pass: "dindinlk.1",
            };
            let _ = user.login_in(mut db);
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
