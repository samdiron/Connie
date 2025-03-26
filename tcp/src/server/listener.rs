use std::{io, thread};
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::time::{Duration, Instant};
use common_lib::cheat_sheet::{LOCAL_IP, TCP_MAIN_PORT};
use common_lib::public_ip;
use lib_db::jwt::DURATION;
use lib_db::server::server_struct::Server;
use lib_db::sqlite::sqlite_host::SqliteHost;
use lib_db::types::PgPool;
use common_lib::log::{debug, info, warn};
use common_lib::tokio::{net::TcpListener, task};
use tokio::net::TcpStream;
use tokio::task::JoinHandle;
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





#[allow(unused_assignments)]
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
    let mut handles: Vec<JoinHandle<()>> = vec![];
    let mut time_for_request_handle = Instant::now();
    let standard_clean_up_tls_dur = Instant::now();
    let wait1day =  Duration::from_secs(DURATION); // DURATION == 1 day
    let standard_wait = Duration::from_secs(600); // 10 min
    let mut impropertls: u32 = 0;
    loop {
        if time_for_request_handle.elapsed() >= standard_wait && handles.len() > 0usize {
            let total_tasks = handles.len(); 
            info!("TASKCLEANER: {} tasks to check", total_tasks);
            let mut faild_tasks:u64 = 0;
            let mut succesful_tasks:u64 = 0;
            time_for_request_handle+=standard_wait;
            let mut items_to_remove: Vec<usize>= vec![];
            for i in 0usize..total_tasks {
                if handles[i].is_finished() {
                    items_to_remove.push(i);
                }
            }
            let total_items_to_remove = items_to_remove.len();
            if total_items_to_remove > 0usize {
                for i in 1usize..total_items_to_remove+1usize {
                    let index = items_to_remove.remove(total_items_to_remove-i);
                    let handle = handles.remove(index);
                    match handle.await {
                        Ok(..) => {succesful_tasks+=1;}
                        Err(..) => {faild_tasks+=1;}
                    }
                }
                info!("TASKCLEANER: total tasks {total_tasks}/ finished tasks {total_items_to_remove} / succesful tasks {succesful_tasks} / faild tasks {faild_tasks}");
            };
            if standard_clean_up_tls_dur.elapsed() >= wait1day {
                impropertls=0
            }

        }; if impropertls ==  1000_000 {
            let now = Instant::now();
            let dur = standard_clean_up_tls_dur - now;
            warn!("you are being DDoSed and i don't wanna deal with this i will sleep for {}s. goodnight (っ- ‸ - ς)",dur.as_secs());
            thread::sleep(dur);
        }
        match socket.accept().await {
            Ok(stream) => {
                let inner_p = pool.clone();
                let inner_allow_new_users = allow_new_users.clone();
                let sqlite_host = sqlite_host.clone();
                let addr = stream.1;
                let inner_acceptor = acceptor.clone();
                let tls = tls_acceptor(inner_acceptor, stream.0).await;
                if tls.is_ok() {
                let handle = task::spawn(async move {
                    let tls = tls.unwrap();
                    let stream = (tls, addr);
                    info!("client: {}", &addr);
                    match handle(stream, inner_p, inner_allow_new_users, sqlite_host).await {
                        Ok(res) => {
                            if res == 0 {info!("a client was handled")}
                            else if res == 1 {
                                info!("client was lost");
                            }
                        },
                        Err(e) => {debug!("a client request faild: {:#?}", e)},
                    }
                    });
                handles.push(handle);
                } else {
                    warn!("client with improper tls addres: {addr}");
                    impropertls+=1;
                }
            }Err(e) => {
                warn!("there was an err while accepting a client : {:#?}", e)
            }
        }
    } 

}

