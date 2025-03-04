use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use common_lib::cheat_sheet::{LOCAL_IP, PUB_IP, TCP_MAIN_PORT};
use lib_db::server::server_struct::Server;
use lib_db::sqlite::sqlite_host::SqliteHost;
use lib_db::types::PgPool;
use common_lib::log::{debug, info, warn};
use common_lib::tokio::{net::TcpListener, task};
use crate::consts::{NET_STATUS, NEW_USERS, PORT, USE_IP, USE_PORT};
use crate::server::handle_client::handle;

pub async fn bind(pool: PgPool, ident: Server) {
    let ip = if *USE_IP.lock().unwrap() == NET_STATUS {
        debug!("server will listen on a custom ip");
        let ip = "0.0.0.0";
        let ipaddr = IpAddr::from_str(ip).expect("not a valid ip addr");
        ipaddr
    } else {
        debug!("server will listen on a default ip");
        let pri = LOCAL_IP.clone();
        pri
    };
    let port = if *USE_PORT.lock().unwrap() == 1 {
        let port = *PORT.lock().unwrap();
        port
    } else {
        TCP_MAIN_PORT.clone()
    };
    let addr = SocketAddr::new(ip, port);
    let socket = TcpListener::bind(&addr).await.unwrap();
    info!("listener on {:?}", addr);
    let allow_new_users = if *NEW_USERS.lock().unwrap() == 1 {
        true
    }else {false};
    let sqlite_host = SqliteHost {
        name: ident.name,
        cpid: ident.cpid,
        host: ident.host,
        port,
        pub_ip: PUB_IP.to_string(),
        pri_ip:LOCAL_IP.to_string(),
    };
    loop {
        match socket.accept().await {
            Ok(stream) => {
                info!("client: {}", stream.1.clone());
                let inner_p = pool.clone();
                let inner_allow_new_users = allow_new_users.clone();
                let sqlite_host = sqlite_host.clone();
                task::spawn(async move {
                    match handle(stream, inner_p, inner_allow_new_users, sqlite_host).await {
                        Ok(..) => {info!("a client was handled")},
                        Err(_) => {debug!("a cleint request faild")},
                    }
                });
            }Err(e) => {
                warn!("there was an err while accepting a client : {:#?}", e)
            }
        }
    } 

}

