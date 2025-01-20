use std::net::SocketAddr;

use common_lib::cheat_sheet::{TCP_MAIN_PORT,LOCAL_IP};
use lib_db::types::PgPool;
use tokio::net::{TcpListener, TcpStream};


async fn handle(stream: &mut (TcpStream, SocketAddr), pool: &PgPool) {

}

pub async fn listener(pool: &PgPool) {
    let ip = LOCAL_IP.clone();
    let addr = SocketAddr::new(ip, TCP_MAIN_PORT.clone());
    let socket = TcpListener::bind(addr).await.unwrap();
    loop {
        match socket.accept().await {
            Ok(mut stream) => {
                handle(&mut stream, pool).await;
            }
            _ => {}
        }
    } 

}

