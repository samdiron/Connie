use std::{path::PathBuf, str::FromStr};

use common_lib::{log::{debug, error}, path::DATA_DIR};
use lib_db::{
    database::get_conn,
    media::server_side::{
        in_storage_files,
        in_storage_size
    },
    server::host::get_host_info
};
use lib_start::{
    file_checker,
    tcp::server_config::{
        ALL_AV_NET,
        PRI_NET
    }
};
use tcp::{
    consts::{
        NET_STATUS,
        NEW_USERS,
        PORT,
        PRIVATE_STATUS,
        USE_IP,
        USE_PORT
    },
    server::listener::bind
};

use crate::{
    get_pass, 
    Commands
};

pub async fn handle_cli_bind(command: Commands) {
    match command {
                Commands::BIND {
            default,
            ip,
            secret,
            port,
            server
        } => {
            if default.is_some() && default.unwrap() {
                let config = lib_start::tcp::server_config::get_server_config()
                    .await
                    .unwrap();
                
                let pool =  get_conn().await.unwrap();
                let _res = get_host_info(
                    &config.default_server.name,
                    &config.default_server.password,
                    &pool
                ).await;

                if config.new_users {
                    let mut new = NEW_USERS.lock().unwrap();
                    *new = 1
                };
                match config.default_network.as_str() {
                    ALL_AV_NET => {
                        let mut _use_it = USE_IP.lock()
                            .expect("could nto lock port");
                        *_use_it = NET_STATUS
                    } 
                    PRI_NET => {
                        let mut _use_it = USE_IP.lock()
                            .expect("could nto lock port");
                        *_use_it = PRIVATE_STATUS

                    }
                    _=> {error!("unexpected network from config")}
                }
                let ip_status = *USE_IP.lock().unwrap();
                let port = *USE_PORT.lock().unwrap();
                debug!("ip status {ip_status}, port status {port}");
                let files_size = in_storage_size(
                    &pool,
                    &config.default_server.cpid
                ).await;
                let files_path = in_storage_files(
                    &pool,
                    &config.default_server.cpid
                ).await;
                let dir = PathBuf::from_str(DATA_DIR).unwrap();
                file_checker(&dir, &files_path, files_size).await;
                bind(pool, config.default_server).await
                
            } else if server.is_some() {
                let _pool =  get_conn().await.unwrap();
                let pool = &_pool;
                let mut passwd = String::new();
                let server = server.unwrap();
                get_pass(&mut passwd, &server);

                let _res = get_host_info(&server, &passwd, pool).await;
                if _res.is_err() {
                    panic!("not a valid server");
                }

                if let Some(ip) = ip {
                    let mut _use_it = *USE_IP.lock()
                        .expect("could nto lock port");
                    _use_it = NET_STATUS
                    
                }
                if let Some(port) = port {
                    let mut _port_mutex = *PORT.lock()
                        .expect("could not lock port");
                    _port_mutex = port;

                    let mut _use_it = *USE_PORT.lock()
                        .expect("could nto lock port");
                    _use_it = 1
                    
                }
                if let Some(secret) = secret {
                    let mut _word_mutex = *lib_db::jwt::MUTEX_SECRET_WORD
                        .lock().unwrap();
                    _word_mutex = secret.as_str()
                }

                
                bind(_pool, _res.unwrap()).await;
            }
        }

        _ => {}
    } 
}
