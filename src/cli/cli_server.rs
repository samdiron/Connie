use std::process::exit;

use common_lib::{cheat_sheet::TCP_MAIN_PORT, gethostname::gethostname, sysinfo};
use lib_db::{database::get_conn, server::server_struct::Server};
use lib_start::tcp::server_config::{self, ServerIdent, ALL_AV_NET, POSTGRES, PRI_NET};

use crate::{cli::Commands, get_new_pass};


pub async fn handle_cli_server(command: Commands) {
    match command {
         Commands::SERVER {
            new,
            default_machine,
            port, 
            new_users,
            net_space,
            update,
            ip,
            name,
            host,
            max_conn 
        } => {
            let pool =  get_conn().await.unwrap();
            let pool = &pool;
            
            let net_space = if net_space.is_some() {
                net_space.unwrap()
            } else {PRI_NET.to_string()};
            if net_space.as_str() != PRI_NET&&
                net_space.as_str() != ALL_AV_NET {
                println!("--net-space should be one of [{PRI_NET},{ALL_AV_NET}]");
                exit(1)

            }

            if new.is_some() && update.is_some() {
                println!("don't be crazy");
            }
                if new.is_some() {
                if host.is_none(){
                    
                    let mut sys = sysinfo::System::new();
                    sys.refresh_all();
                    let memory = sys.total_memory();
                    
                    println!("memory: {}", memory);
                    let mut password = String::new();
                    get_new_pass(&mut password, &name);
                    

                    let string_host = gethostname()
                        .to_str()
                        .unwrap()
                        .to_string();
                    let max_conn = if max_conn.is_some() {
                        let int = max_conn.unwrap();
                        int
                        
                    }else {
                        80
                    };
                    let ip = if ip.is_some() {
                        let ip = ip.unwrap();
                        ip.to_string()
                    } else {
                        common_lib::cheat_sheet::LOCAL_IP.to_string()
                    };
                    let server = Server {
                        ip,
                        cpid: String::new(),
                        name,
                        host: string_host,
                        memory: memory as i64,
                        max_conn,
                        password

                    };
                    let _server = server.create(pool).await.unwrap();
                    if default_machine.is_some() {
                        let port: u16 = if port.is_some() {
                            let p = port.unwrap();
                            p
                        } else {
                            TCP_MAIN_PORT
                        };
                        let new_users = if new.is_some() {true} else {false};
                        let serveri = ServerIdent {
                            default_server: _server,
                            default_port: port,
                            default_database: POSTGRES.to_string(),
                            default_network: net_space,
                            new_users
                        };
                        server_config::ServerIdent::create_config(serveri).await;

                    }


                    
                    println!("server has been create\n will exit now ");
                    exit(0)
                    
                }  
            }
        }
        _=> {}
    }
}



