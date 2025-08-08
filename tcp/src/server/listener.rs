use std::sync::{Arc, LazyLock, Mutex};
use std::{io, thread};
use std::str::FromStr;
use std::net::{IpAddr, SocketAddr};
use std::time::{Duration, Instant};


use lib_db::types::PgPool;
use lib_db::jwt::DURATION;
use lib_db::server::server_struct::Server;
use lib_db::sqlite::sqlite_host::SqliteHost;
use lib_db::server::host::get_host_public_files;

use common_lib::public_ip;
use common_lib::path::PUBLIC_DATA_DIR;
use common_lib::log::{debug, info, warn};
use common_lib::cheat_sheet::{LOCAL_IP, TCP_MAIN_PORT};

use tokio::task;
use tokio::task::JoinHandle;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use tokio_rustls::TlsAcceptor;
use tokio_rustls::server::TlsStream;

use crate::types::RQM;
use crate::consts::{NET_STATUS, NEW_USERS, USE_IP};
use crate::common::request::SERVER_WILL_NOT_ALLOW_NOTLS;

use crate::server::config::make_config;
use crate::server::handle_client::{handle, raw_handle};

//runtime
use crate::server::runtime::generate_log_templates;
use crate::server::runtime::logs::unauthorized_client_log;
use crate::server::runtime::file_checks::clean_unfinished_files;
use crate::server::runtime::public_files::{
    new_pub_files_process,
    pub_files_process
};


pub static ALL_REQUESTS: LazyLock<Mutex<Vec<RQM>>> = LazyLock::new(||{
    let vector: Mutex<Vec<RQM>> = Mutex::new(vec![]);
    vector
});

pub async fn bind(
    pool: PgPool,
    ident: Server,
    port: u16,
    allow_notls: bool
) {
        
    info!("allow notls status is {}", allow_notls);
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
    let port = if port == 0 {
        TCP_MAIN_PORT
    } else {
        port
    };
    let addr = SocketAddr::new(ip, port);
    let socket = TcpListener::bind(&addr).await.unwrap();
    info!("listener on {:?}", addr);
    
    let allow_new_users = if *NEW_USERS.lock().unwrap() == 1 {
        debug!("new users will be allowed");
        true
    }else {
        debug!("new users will not be allowed");
        false
    };
    let current_machine_pub_ip = public_ip::addr().await.unwrap();
    let local_cpid = ident.cpid.clone();
    let sqlite_host = SqliteHost {
        name: ident.name,
        cpid: ident.cpid,
        host: ident.host,
        port,
        pub_ip: current_machine_pub_ip.to_string(),
        pri_ip:LOCAL_IP.to_string(),
    };
    debug!("will fetch public files ");
    let db_pub_files = get_host_public_files(&sqlite_host, &pool)
        .await.unwrap();
    if !db_pub_files.is_empty() {
        pub_files_process(
            db_pub_files,
            PUBLIC_DATA_DIR,
            &sqlite_host.cpid,
            &pool
        ).await.unwrap();
    }else {
        new_pub_files_process(
            PUBLIC_DATA_DIR,
            &sqlite_host.cpid,
            &pool
        ).await.unwrap();
    }

    let config = make_config();
    let acceptor = TlsAcceptor::from(config);
    let mut handles: Vec<JoinHandle<()>> = vec![];


    let mut n_all_time_requests = 0u64;
    let mut n_current_requests = 0u64;
    let mut succesful_requests = 0u64;
    let mut failed_requests = 0u64;

    // timers and counters
    // cleaning time
    let mut ct = Instant::now();
    // admin pause
    let mut ap = Instant::now();
    let standard_clean_up_tls_dur = Instant::now();
    let wait1day =  Duration::from_secs(DURATION); // DURATION == 1 day
    let standard_wait = Duration::from_secs(300); // 5 min
    let mut impropertls: u32 = 0;

    loop {
        // admin loop 
        if handles.is_empty() && ap.elapsed() > standard_wait {
            ap+=standard_wait;
        }

        // cleaner loop 
        if (ct.elapsed() >= standard_wait && !handles.is_empty()) || handles.len() >= 10usize {
            if ALL_REQUESTS.lock().is_ok(){
                let size = ALL_REQUESTS.lock().unwrap().len();
                debug!("all request admin copies: {size}");
            }
            admin_loop(
                &mut handles,
                &mut impropertls,
                &local_cpid,
                &pool,
                &mut ct,
                &wait1day,
                &standard_wait,
                &standard_clean_up_tls_dur,
                &mut succesful_requests,
                &mut failed_requests,
                &mut n_current_requests

            ).await;
        }; 
        if impropertls ==  100_000 {
            let now = Instant::now();
            let dur = standard_clean_up_tls_dur - now;
            warn!("you are being DDoSed and i don't wanna deal with this i will sleep for {}s. goodnight (っ- ‸ - ς)",dur.as_secs());
            thread::sleep(dur);
        }

        // listener
        match socket.accept().await {
            Ok(stream) => {
                n_all_time_requests+=1;
                let inner_p = pool.clone();
                let inner_allow_new_users = allow_new_users.clone();
                let sqlite_host = sqlite_host.clone();
                let addr = stream.1;
                let mut stream = stream.0;
                let no_tls = stream.read_u8().await;

                let no_tls = if no_tls.is_ok() {
                    no_tls.unwrap()
                } else { 0 };
                
                if no_tls == 1 && allow_notls {
                    serving_no_tls_request(
                        stream,
                        addr,
                        inner_p,
                        inner_allow_new_users,
                        sqlite_host,
                        &mut handles
                    ).await;
                } else if !allow_notls && no_tls == 1 {
                    let res = stream.write_u8(SERVER_WILL_NOT_ALLOW_NOTLS).await;
                    if res.is_ok() {res.unwrap()};
                }
                else {
                    info!("notls client");
                    let inner_acceptor = acceptor.clone();
                    let tls = tls_acceptor(inner_acceptor, stream).await;
                    if tls.is_ok() {
                        serving_tls_request(
                            tls.unwrap(),
                            addr,
                            inner_p,
                            inner_allow_new_users,
                            sqlite_host,
                            &mut handles
                        ).await;
                    } else {
                        warn!("client with improper tls addres: {}", &addr);
                        let _ = unauthorized_client_log(addr).await.unwrap();
                        impropertls+=1;
                    }
                };
            }Err(e) => {
                warn!("there was an err while accepting a client : {:#?}", e)
            }
        }
    } 

}

async fn tls_acceptor(
    acceptor: TlsAcceptor,
    raw_stream: TcpStream
) -> Result<TlsStream<TcpStream>, io::Error> {
    let s = acceptor.accept(raw_stream).await?;
    Ok(s)
}

async fn serving_tls_request(
    tls: TlsStream<TcpStream>,
    addr: SocketAddr,
    inner_p: PgPool,
    inner_allow_new_users: bool,
    sqlite_host: SqliteHost,
    handles: &mut Vec<JoinHandle<()>>
){
    let task_handle = task::spawn(async move {
    let stream = (tls, addr);
    info!("client: {}", &addr);
    match handle(
            stream,
            inner_p,
            inner_allow_new_users,
            sqlite_host
        ).await {
        Ok(res) => {
            if res.0 == 0 {info!("a client was handled")}
            else if res.0 == 1 {
                info!("a client was lost");
            } else if res.0 == 44 {
                let err = res.1.unwrap();

                let filename = generate_log_templates::client_cpid_not_match(&err);
                warn!("suspicious activity see full log at {}", filename);
            }
        },
        Err(e) => {debug!("a client request faild: {:#?}", e)},
    }
    });
    handles.push(task_handle);

}

async fn serving_no_tls_request(
    mut stream: TcpStream,
    addr: SocketAddr,
    inner_p: PgPool,
    inner_allow_new_users: bool,
    sqlite_host: SqliteHost,
    handles: &mut Vec<JoinHandle<()>>
){
    let res = stream.write_u8(0).await;
    if res.is_ok() {
        res.unwrap();
    };
    let task_handle = task::spawn(async move {
    info!("client: {}", &addr);
    match raw_handle(
            (stream, addr),
            inner_p,
            inner_allow_new_users,
            sqlite_host
        ).await {
        Ok(res) => {
            if res.0 == 0 {info!("a client was handled")}
            else if res.0 == 1 {
                info!("a client was lost");
            } else if res.0 == 44 {
                let err = res.1.unwrap();

                let filename = generate_log_templates::client_cpid_not_match(&err);
                warn!("a suspicious activity see full log at {}", filename);
            }
        },
        Err(e) => {debug!("a client request faild: {:#?}", e)},
    }
    });
    handles.push(task_handle);

}

async fn admin_loop(
    handles: &mut Vec<JoinHandle<()>>,
    impropertls: &mut u32,
    local_cpid: &String,
    pool: &PgPool,
    ct: &mut Instant,
    wait1day: &Duration,
    standard_wait: &Duration,
    standard_clean_up_tls_dur: &Instant,
    failed_tasks: &mut u64,
    succesful_tasks: &mut u64,
    current_requests: &mut u64,
) {
    let start = Instant::now();
    let total_tasks = handles.len(); 
    debug!("TASKCLEANER: {} tasks to check", total_tasks);
    *ct = ct.clone() + standard_wait.clone();
    let mut items_to_remove: Vec<usize>= vec![];
    for i in 0usize..total_tasks {
        if handles[i].is_finished() {
            items_to_remove.push(i);
        }
    }
    let total_items_to_remove = items_to_remove.len();
    if total_items_to_remove > 0usize {
        for i in 1usize..total_items_to_remove+1usize {
            let index = items_to_remove
                .remove(total_items_to_remove-i);
            let handle = handles.remove(index);
            match handle.await {
                Ok(..) => {*succesful_tasks+=1;}
                Err(..) => {*failed_tasks+=1;}
            }
        }
        debug!("TASKCLEANER: total tasks {total_tasks}/ finished tasks {total_items_to_remove} / succesful tasks {succesful_tasks} / faild tasks {failed_tasks}");
    };
    if handles.is_empty() && *failed_tasks > 0 {
        clean_unfinished_files(local_cpid, pool).await;
    }
    if &standard_clean_up_tls_dur.elapsed() >= wait1day {
        *impropertls= 0u32;
    }
    *current_requests = handles.len() as u64;
    let end = start.elapsed();
    debug!("TASKCLEANER: took {} ms", end.as_millis());
}
