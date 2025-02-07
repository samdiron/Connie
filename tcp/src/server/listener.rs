use std::net::SocketAddr;
use common_lib::cheat_sheet::{TCP_MAIN_PORT,LOCAL_IP};
use lib_db::types::PgPool;
use log::warn;
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

