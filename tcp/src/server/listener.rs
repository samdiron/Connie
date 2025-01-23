use std::io::Read;
use std::net::SocketAddr;
use common_lib::cheat_sheet::{TCP_MAIN_PORT,LOCAL_IP};
use lib_db::types::PgPool;
use log::warn;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{net::TcpListener, task};
use tokio::net::TcpStream;


async fn handle(st: (TcpStream, SocketAddr), pool: PgPool) -> std::io::Result<()> {
    let _p = pool;
    let mut buf = String::new();
    let mut stream = st.0;
    let addr = st.1;
    println!("client: {addr} is being served");
    stream.read_to_string(&mut buf).await?;
    let mut f = std::fs::File::open("../../../res.html")?;
    let mut buff = vec![0; 4000];
    let file_size = f.read_to_end(&mut buff)?;
    stream.write_all(&buff).await?;
    println!("file_size: {file_size}");
    Ok(())

}


pub async fn bind(pool: PgPool) {
    let ip = LOCAL_IP.clone();
    let addr = SocketAddr::new(ip, TCP_MAIN_PORT.clone());
    let socket = TcpListener::bind(addr).await.unwrap();
    loop {
        match socket.accept().await {
            Ok(stream) => {
                println!("stream ");
                let inner_p = pool.clone();
                match task::spawn(async{
                            match handle(stream, inner_p).await {
                                Ok(..) => {},
                                Err(..) => {},
                            }
                            }).await {
                    Ok(_) => {
                        println!("a client was server");
                    },
                    Err(_) => {},
                };
                
            }
            Err(e) => {
                warn!("an error ocured while accepting a stream");
                eprintln!("an error ocured while accepting a stream: {e}");
            }
        }
    } 

}

