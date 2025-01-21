use std::os::unix::thread;
use std::thread::available_parallelism;
use std::{net::SocketAddr, sync::mpsc::channel};
use std::sync::mpsc;
use common_lib::cheat_sheet::{TCP_MAIN_PORT,LOCAL_IP};
use lib_db::types::PgPool;
use log::{info, warn};
use tokio::{io::{AsyncReadExt, Interest}, net::{TcpListener, TcpStream}};

use super::pool::ThreadPool;


async fn handle(st: (TcpStream, SocketAddr), pool: &PgPool) {
    let mut buf = String::new();
    let mut stream = st.0;
    let addr = st.1;
    info!("client: {addr} is being served");
    stream.read_to_string(&mut buf).await.unwrap();

}

pub async fn listener(pool: &PgPool) {
    let size = usize::from(available_parallelism().unwrap());
    let threads = ThreadPool::new(size);

    let ip = LOCAL_IP.clone();
    let addr = SocketAddr::new(ip, TCP_MAIN_PORT.clone());
    let socket = TcpListener::bind(addr).await.unwrap();
    loop {
        match socket.accept().await {
            Ok(stream) => {
                let f = handle(stream, pool);
                f
                threads.execute(handle(stream, pool));

                
            }
            Err(e) => {
                warn!("an error ocured while accepting a stream");
                eprintln!("an error ocured while accepting a stream: {e}");
            }
        }
    } 

}

