use std::io;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use common_lib::cheat_sheet::{LOCAL_IP, TCP_MAIN_PORT};
use common_lib::public_ip;
use lib_db::server::server_struct::Server;
use lib_db::sqlite::sqlite_host::SqliteHost;
use lib_db::types::PgPool;
use common_lib::log::{debug, info, warn};
use common_lib::tokio::{net::TcpListener, task};
use tokio::net::TcpStream;
use tokio_rustls::server::TlsStream;
use tokio_rustls::TlsAcceptor;
use crate::consts::{NET_STATUS, NEW_USERS, PORT, USE_IP, USE_PORT};
use crate::server::config::make_config;
use crate::server::handle_client::handle;





async fn tls_acceptor(
    acceptor: TlsAcceptor,
    raw_stream: TcpStream
) -> Result<TlsStream<TcpStream>, io::Error> {
    let s = acceptor.accept(raw_stream).await?;
    Ok(s)
}





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
    let me_pub_ip = public_ip::addr().await.unwrap();
    let sqlite_host = SqliteHost {
        name: ident.name,
        cpid: ident.cpid,
        host: ident.host,
        port,
        pub_ip: me_pub_ip.to_string(),
        pri_ip:LOCAL_IP.to_string(),
    };
    let config = make_config();
    let acceptor = TlsAcceptor::from(config);
    loop {
        match socket.accept().await {
            Ok(stream) => {
                info!("client: {}", &stream.1);
                let inner_p = pool.clone();
                let inner_allow_new_users = allow_new_users.clone();
                let sqlite_host = sqlite_host.clone();
                let addr = stream.1;
                let tls = tls_acceptor(acceptor.clone(), stream.0)
                    .await
                    .expect("could not accsept tls");
                let stream = (tls, addr);
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

