// #![allow(unused_assignments)]
#![allow(unused_variables)]

use std::{fs::remove_file, io::{stdout, Write}, net::IpAddr, path::PathBuf, process::exit};
use env_logger;
use common_lib::{cheat_sheet::TCP_MAIN_PORT, gethostname::gethostname, log::{debug, error}, path::SQLITEDB_PATH};
use lib_db::{
    database::{get_conn, DB_CONN}, server::{host::get_host_info, server_struct::Server}, sqlite::{self, sqlite_user::ShortUser}, user::{user_jwts::get_jwt, user_struct::{fetch, User}}
};
use lib_start::{certs, tcp::server_config::{ServerIdent, ALL_AV_NET, POSTGRES, PRI_NET}};
use lib_start::tcp::server_config;
use tcp::{client::{client::{client_process, signup_process}, fetcher}, consts::{NET_STATUS, NEW_USERS, PORT, PRIVATE_STATUS, USE_IP, USE_PORT}, server::listener::bind, types::{POST, RQM}};
use common_lib::rpassword::read_password;
use common_lib::sysinfo;
use clap::{command, Parser, Subcommand};
use tokio::{fs::File, io::AsyncWriteExt};




//NOTE: for this progrm to start you have to write your postgres connection url
//like this postgres://db_user:password_for_the_user@ip:port/database_name
//postgres default port is 5432, and ip by default is localhost
//in the /Connie/etc/db_conn; file

#[derive(Debug,Parser)]
#[command(version = "0.2beta", about = "a web server in rust for more info visit https://github.com/samdiron/Connie")]
#[command(disable_help_flag = true)]
struct Cli {
    
    #[arg(long)]
    db: Option<String>,

    #[command(subcommand)]
    config: Option<Commands>,

    #[arg(long, short, default_value="false")]
    tui: Option<bool>,

    #[arg(long, short, default_value="1")]
    verbose: Option<u8>,

    #[arg(long, short, default_value = "false")]
    generate_certs: Option<bool>,

}




#[derive(Debug,Subcommand)]
enum Commands {

    DEV {
        #[arg(long,short)]
        tls_config: Option<bool>,
        #[arg(long,short)]
        env_logger: Option<bool>,
    },
     
    BIND {
        
        #[arg(long, short)]
        default: Option<bool>,

        #[arg(long, short)]
        ip: Option<String>,
        
        #[arg(long, short)]
        server: Option<String>,
        
        #[arg(long)]
        secret: Option<String>,

        #[arg(long, short)]
        port: Option<u16>,

    },


    REQUEST {

        
        #[arg(long, short)]
        user: String,

        #[arg(long)]
        signup: Option<bool>,
        
        #[arg(long, short)]
        ip: Option<IpAddr>,
        
        #[arg(long)]
        port: Option<u16>,

        #[arg(long, short)]
        host: Option<String>,
        
        #[arg(long, short)]
        fetch_files: Option<bool>,

        #[arg(long, short)]
        get: Option<PathBuf>,
        
        #[arg(long, short)]
        post: Option<PathBuf>,
        #[arg(long, short, default_value="true")]
        create_checksum: Option<bool>,

    },
    

    MULTICAST {
                
        #[arg(long, short)]
        ip: Option<String>,
    },



    SERVER {
        #[arg(long)]
        new: Option<bool>,

        #[arg(long)]
        update: Option<bool>,

        #[arg(long, short)]
        default_machine: Option<bool>,

        #[arg(long)]
        net_space: Option<String>,

        #[arg(long)]
        new_users: Option<bool>,
        

        #[arg(long,short)]
        port: Option<u16>,

        #[arg(long, short)]
        ip: Option<IpAddr>,

        #[arg(long, short)]
        name: String,

        #[arg(long, short)]
        max_conn: Option<i16>,
         
        #[arg(long)]
        host: Option<String>,

    },

    DB {
         
        #[arg(long)]
        test: Option<bool>,

        #[arg(long)]
        delete_conn: Option<bool>,

        #[arg(long, short)]
        migrations: Option<bool>,

        #[arg(long, short)]
        connection: Option<String>,

    },


    User {
        #[arg(long)]
        new: Option<bool>,

        #[arg(long)]
        update: Option<bool>,
        
        #[arg(long)]
        host: Option<String>,

        #[arg(long, short)]
        signup: Option<bool>,

        #[arg(long)]
        admin: Option<bool>,

        #[arg(long, short)]
        name: String,

        #[arg(long, short)]
        username: String,

        #[arg(long, short)]
        email: String,
    }
}


fn get_pass(password: &mut String, name: &str) {
        print!("enter password for {} : ", name);
        stdout().flush().expect("could not flush");
        let password1 = read_password().unwrap();
        *password = password1;
}


fn get_new_pass(password: &mut String, name: &str) {
    for i in 0..2 {
        print!("enter password for {} : ", name);
        stdout().flush().expect("could not flush");
        let password1 = read_password().unwrap();
        print!("confirm password");
        stdout().flush().expect("could not flush");
        let password2 = read_password().unwrap();
        if password2 == password1 && (password1.is_empty() == false) {
            *password = password1;
            break;
        }
        if password1 != password2 {
            println!("password do not match");
            if i == 2 {
                println!("3 times, will exit now");
                exit(1);
            }
        }
    };

}

async fn config_handle(command: Commands ) {
    match command {
        Commands::User { new, signup ,update, host, admin, name, username, email } => {
            let pool =  get_conn().await.unwrap();
            let pool = &pool;
            if new.is_some() && new.unwrap()  {
                let mut password = String::new(); 
                let string_host: String;
                get_new_pass(&mut password, name.as_str());
                if host.is_none() {
                    let h = gethostname();
                    let str_binding = h.to_str().unwrap();
                    string_host = str_binding.to_string();
                }else {
                    let h = host.unwrap();
                    string_host = h;
                }
                let _user_ = User {
                    cpid: String::new(),
                    name,
                    username,
                    email,
                    password,
                    host: string_host
                };
                let _user = _user_.create(pool).await.unwrap();

                if admin.is_some() && admin.unwrap() {
                    // empty for now
                }

            } else if signup.is_some() && signup.unwrap() {
                let mut password = String::new(); 
                get_new_pass(&mut password, &name);
                let ShortUser {
                    name,
                    username,
                    email,
                    password
                    
                }
            }
        } 
        Commands::SERVER { new, default_machine, port, new_users, net_space, update, ip, name, host, max_conn } => {
            let pool =  get_conn().await.unwrap();
            let pool = &pool;
            let net_space = if net_space.is_some() {net_space.unwrap()} else {PRI_NET.to_string()};
            if net_space.as_str() != PRI_NET && net_space.as_str() != ALL_AV_NET {
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
                    

                    let string_host = gethostname().to_str().unwrap().to_string();
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
        Commands::BIND {default, ip, secret, port, server } => {
            if default.is_some() && default.unwrap() {
                let config = lib_start::tcp::server_config::get_server_config()
                    .await
                    .unwrap();
                
                let pool =  get_conn().await.unwrap();
                let _res = get_host_info(&config.default_server.name, &config.default_server.password, &pool).await;

                if config.new_users {
                    let mut new = NEW_USERS.lock().unwrap();
                    *new = 1
                };
                match config.default_network.as_str() {
                    ALL_AV_NET => {
                        let mut _use_it = USE_IP.lock().expect("could nto lock port");
                        *_use_it = NET_STATUS
                    } 
                    PRI_NET => {
                        let mut _use_it = USE_IP.lock().expect("could nto lock port");
                        *_use_it = PRIVATE_STATUS

                    }
                    _=> {error!("unexpected network from config")}
                }
                let ip_status = *USE_IP.lock().unwrap();
                let port = *USE_PORT.lock().unwrap();
                debug!("ip status {ip_status}, port status {port}");
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
                    let mut _use_it = *USE_IP.lock().expect("could nto lock port");
                    _use_it = NET_STATUS
                    
                }
                if let Some(port) = port {
                    let mut _port_mutex = *PORT.lock().expect("could not lock port");
                    _port_mutex = port;

                    let mut _use_it = *USE_PORT.lock().expect("could nto lock port");
                    _use_it = 1
                    
                }
                if let Some(secret) = secret {
                    let mut _word_mutex = *lib_db::jwt::MUTEX_SECRET_WORD.lock().unwrap();
                    _word_mutex = secret.as_str()
                }

                
                bind(_pool, _res.unwrap()).await;
            }
        }
        Commands::DB { migrations, connection, delete_conn, test } => {
            if let Some(conn) = connection {
                let mut f = File::create_new(DB_CONN)
                    .await
                    .expect("this command creates a new file in /opt/Connie/conf it needs to be executed by root ");
                f.write_all(conn.as_bytes())
                    .await
                    .unwrap();
            }
            if let Some(delete_conn) = delete_conn {
                if delete_conn == true {
                    remove_file(DB_CONN).expect("this command deletes a file this will need root");
                    println!("deleted {DB_CONN} file");
                    exit(0)
                }
            }
            if let Some(migrations) = migrations {
                let pool =  get_conn().await.unwrap();
                let pool = &pool;
                let spool = sqlite::get_sqlite_conn(&SQLITEDB_PATH.to_owned()).await.unwrap();
                if migrations == true {
                    lib_db::database::migrate(pool).await.unwrap();
                    lib_db::sqlite::migration(&spool).await.unwrap();
                }
            }
            if let Some(test) = test {
                if test == true {
                    let _pool = get_conn().await.expect("can't connect to db");
                    println!("db connection valid");
                    exit(0)
                }
            } 
        }
        Commands::REQUEST { ip, signup ,port, host, get, post, create_checksum, fetch_files ,user} => {    
            let _pool = sqlite::get_sqlite_conn(&SQLITEDB_PATH.to_string()).await.unwrap();
            let pool = &_pool;
          
            let mut passwd = String::new();
            get_pass(&mut passwd, user.as_str());
            let usr = sqlite::sqlite_user::fetch_sqlite_user(user, passwd, pool).await.expect("could not fetch that user");
            
            debug!("user cpid: {} , name: {}", usr.cpid, usr.usrname);
            if fetch_files.is_some() && port.is_some() && ip.is_some()  &&  host.is_some() {
                let jwt = sqlite::sqlite_jwt::get_jwt(&host.unwrap(), &usr.cpid, pool).await.unwrap();
                fetcher::get_files(usr, ip.unwrap(), port.unwrap(), jwt).await.unwrap();
            } else if host.is_some() && post.is_some() { 
                let host = host.unwrap();
                let create_checksum = create_checksum.unwrap();
                println!("creating a checksum this will take a moment");
                let request: RQM = 
                    if post.is_some() {
                        let r = RQM::create(
                            post.unwrap(),
                            POST.to_string(),
                            usr.cpid.clone(),
                            create_checksum
                        ).await.unwrap();
                        r
                    } else {println!("you did not enter a request to exec"); exit(0)};
                let res = client_process(host, ip, _pool, usr, request).await.unwrap();
                println!("done {}", res);
            }
        }
        _ => {}
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    //start of the program 
    let _cli = Cli::parse();
    
    if _cli.generate_certs.is_some() && _cli.generate_certs.unwrap() {
        certs::generate_certs();
    }
    if _cli.verbose.is_some() {
        match _cli.verbose.unwrap() {
            0 => {
            env_logger::Builder::new()
                    .parse_filters("WARN")
                    .parse_filters("ERROR")
                    .init();
            }
            1 => {
            env_logger::Builder::new()
                    .parse_filters("WARN")
                    .parse_filters("ERROR")
                    .parse_filters("INFO")
                    .init();
            }
            2 => {
            env_logger::Builder::new()
                    .parse_filters("trace")
                    .init();
            }
            _ => {}
        }

    }
    
    
    if let Some(command) = _cli.config {
        config_handle(command).await;
    }else {
        println!("not now");
    }
    
    

    //end of the program
}
