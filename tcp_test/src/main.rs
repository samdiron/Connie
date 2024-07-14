use std::{
    //intrinsics::discriminant_value,
    io,
    net::{TcpListener, TcpStream},
    //u8,
};
/*use tokio::{
    net::{TcpListener, TcpStream},
    stream,
};*/

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    //let state: u8 = 0;
    //let buff = 1;
    //let val = [0, 1, 3].len();
    //stream.write_all(&val.to_be_bytes()?);
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
