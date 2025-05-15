use std::{
    path::PathBuf,
    str::FromStr
};

use common_lib::{
    log::{debug, error},
    path::DATA_DIR
};
use lib_db::{
    database::get_conn,
    jwt::MUTEX_SECRET_WORD,
    server::host::get_host_info,
    media::server_side::{
        in_storage_files,
        in_storage_size
    },
};
use lib_start::{
    file_checker,
    tcp::server_config::{
        ALL_AV_NET,
        PRI_NET
    }
};

use lib_start::tcp::server_config::get_server_config;
use tcp::{
    server::listener::bind,
    consts::{
        NET_STATUS,
        NEW_USERS,
        PRIVATE_STATUS,
        USE_IP,
    },
};

use crate::{
    get_pass, 
    Commands
};

pub async fn handle_cli_bind(command: Commands) {
    match command {
            Commands::BIND {
            ip,
            port,
            users,
            secret,
            server,
            default,
            new_users,
            admin_port,
        } => {

            if secret.is_some() {
                *MUTEX_SECRET_WORD
                    .lock()
                    .unwrap()
                    .lock()
                    .unwrap() = secret.unwrap();
            }

            if let Some(ip) = ip {
                    let mut _use_it = *USE_IP.lock()
                        .expect("could not lock ip");
                _use_it = NET_STATUS
                
            }
            if new_users.is_some() && new_users.unwrap() {
                let mut new = NEW_USERS.lock().unwrap();
                *new = 1
            };



            if default.is_some() && default.unwrap() {
                let config = get_server_config()
                    .await
                    .unwrap();
                 
                let pool =  get_conn().await.unwrap();
                let _res = get_host_info(
                    &config.default_server.name,
                    &config.default_server.password,
                    &pool,
                    true
                ).await;

                if config.new_users {
                    let mut new = NEW_USERS.lock().unwrap();
                    *new = 1
                };
                let port = if port.is_some() {
                    port.unwrap()
                } else {
                    config.default_port
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
                let files_size = in_storage_size(
                    &pool,
                    &config.default_server.cpid
                ).await;

                let files_path = in_storage_files(
                    &pool,
                    &config.default_server.cpid
                ).await;

                debug!("should be files: {}",files_path.len());
                let dir = PathBuf::from_str(DATA_DIR).unwrap();
                file_checker(&dir, &files_path, files_size).await;
                bind(pool, config.default_server, port).await
                
            } else if server.is_some() {
                let _pool =  get_conn().await.unwrap();
                let pool = &_pool;
                let mut passwd = String::new();
                let server = server.unwrap();
                get_pass(&mut passwd, &server);

                let port = if port.is_some() {
                    port.unwrap()
                } else {
                    0
                };

                let _res = get_host_info(
                    &server,
                    &passwd,
                    pool,
                    false
                ).await;
                if _res.is_err() {
                    panic!("not a valid server");
                }

                
                
                bind(_pool, _res.unwrap(), port).await;
            }
        }

        _ => {}
    } 
}
