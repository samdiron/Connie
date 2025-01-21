use std::thread::available_parallelism;
use std::net::SocketAddr;
use common_lib::cheat_sheet::{TCP_MAIN_PORT,LOCAL_IP};
use lib_db::types::PgPool;
use log::warn;
use tokio::net::TcpListener;
use super::pool::ThreadPool;



pub async fn bind(pool: PgPool) {
    let size = usize::from(available_parallelism().unwrap());
    let threads = ThreadPool::new(size, pool);

    let ip = LOCAL_IP.clone();
    let addr = SocketAddr::new(ip, TCP_MAIN_PORT.clone());
    let socket = TcpListener::bind(addr).await.unwrap();
    loop {
        match socket.accept().await {
            Ok(stream) => {
                println!("stream ");
                threads.execute(stream);
                
            }
            Err(e) => {
                warn!("an error ocured while accepting a stream");
                eprintln!("an error ocured while accepting a stream: {e}");
            }
        }
    } 

}

