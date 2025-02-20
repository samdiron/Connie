use std::net::SocketAddr;
use common_lib::cheat_sheet::{TCP_MAIN_PORT,LOCAL_IP};
use lib_db::types::PgPool;
use log::debug;
use tokio::{net::TcpListener, task};
use crate::server::handle_client::handle;




pub async fn bind(pool: PgPool) {
    let ip = LOCAL_IP.clone();
    let addr = SocketAddr::new(ip, TCP_MAIN_PORT.clone());
    let socket = TcpListener::bind(addr.clone()).await.unwrap();
    println!("listener on {:?}", addr);
    loop {
        match socket.accept().await {
            Ok(stream) => {
                println!("client: {}", stream.1.clone());
                let inner_p = pool.clone();
                task::spawn(async{
                    match handle(stream, inner_p).await {
                        Ok(..) => {println!("a client was handled")},
                        Err(_) => {debug!("a cleint request faild")},
                    }
                });
            }Err(e) => {eprintln!("there waqs an err while accepting a client : {:#?}", e)}
        }
    } 

}

