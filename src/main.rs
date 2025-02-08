#![allow(unused_assignments)]

use std::{alloc::System, io::{stdout, Empty, Write}, net::IpAddr, process::exit, str::FromStr};

use gethostname::gethostname;
use lib_db::{
    database::{get_conn, DB_CONN},
    server::{host::get_host_info, server_struct::{get_server, Server}},
    types::PgPool,
    user::user_struct::User
};
use rpassword::read_password;
use tcp::{consts::{IP, PORT, USE_IP, USE_PORT}, server::listener::bind};

use serde::{Deserialize, Serialize};
use clap::{Parser, Subcommand};
use tokio::{fs::File, io::AsyncWriteExt};




//NOTE: for this progrm to start you have to write your postgres connection url
//like this postgres://db_user:password_for_the_user@ip:port/database_name
//postgres default port is 5432, and ip by default is localhost
//in the /Connie/etc/db_conn; file

#[derive(Debug, Deserialize, Serialize, Parser)]
#[command(version = "0.1beta", about = "a web server in rust for more info visit https://github.com/samdiron/Connie")]
struct Cli {
    
    #[arg(long)]
    db: Option<String>,

    #[command(subcommand)]
    config: Option<Commands>

}




#[derive(Debug, Deserialize, Serialize,Subcommand)]
enum Commands {
     
    BIND {

        #[arg(long, short)]
        ip: Option<String>,
        
        #[arg(long, short)]
        server: String,
        
        #[arg(long)]
        secret: Option<String>,

        #[arg(long, short)]
        port: Option<u16>,

    },


    CONNECT {
        
        #[arg(long, short)]
        ip: Option<String>,
        
        #[arg(long, short)]
        port: Option<u16>,
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
        ip: Option<String>,

        #[arg(long, short)]
        name: String,

        #[arg(long, short)]
        max_conn: Option<i64>,
        
        #[arg(long)]
        host: Option<String>,

    },

    DB {
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

async fn config_handle(command: Commands, pool: &PgPool) {
    match command {
        Commands::User { new, update, host, admin, name, username, email } => {
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

            }
        } 
        Commands::SERVER { new, update, ip, name, host, max_conn } => {
            if new.is_some() && update.is_some() {
                println!("don't be crazy");
            }
                if new.is_some() && new.unwrap() {
                if host.is_none() {
                    
                    let mut sys = sysinfo::System::new();
                    sys.refresh_all();
                    let memory = sys.total_memory();
                    
                    println!("memory: {}", memory);
                    let mut password = String::new();
                    get_new_pass(&mut password, name.as_str());
                    

                    let string_host = gethostname().to_str().unwrap().to_string();
                    let max_conn = if max_conn.is_some() {
                        let int = max_conn.unwrap();
                        int
                        
                    }else {
                        80
                    };
                    let ip = if ip.is_some() {
                        let ip_ = ip.unwrap();
                        let check = IpAddr::from_str(ip_.as_str());
                        if check.is_err() {
                            println!("enter a valid ip");
                            exit(1)
                        }
                        ip_
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
                    
                    println!("server has been create\n will exit now ");
                    exit(0)
                    
                }  
            }
        }
        Commands::BIND { ip, secret, port, server } => {
            let mut passwd = String::new();
            get_new_pass(&mut passwd, server.as_str());

            let _res = get_host_info(server, passwd, pool).await;
            if _res.is_err() {
                println!("not a valid server");
            }else {
                println!("valid server");
                exit(1)
            }

            if let Some(ip) = ip {
                let mut _ip_mutex = *IP.lock().expect("could not lock port");
                _ip_mutex = ip.as_str();

                let mut _use_it = *USE_IP.lock().expect("could nto lock port");
                _use_it = 1
                
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

            
            bind(pool.clone()).await;
        }
        Commands::DB { migrations, connection } => {
            if let Some(conn) = connection {
                let mut f = File::create_new(DB_CONN)
                    .await
                    .expect("this command creates a new file in /opt/Connie/conf it needs to be executed by root ");
                f.write_all(conn.as_bytes())
                    .await
                    .unwrap();
            }
            if let Some(migrations) = migrations {
                if migrations == true {
                    lib_db::database::migrate(&pool).await.unwrap();
                }
            } 
        }
        _ => {}
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    //start of the program 

    let _cli = Cli::parse();
    





    let pool =  get_conn().await.unwrap();
    if let Some(command) = _cli.config {
        config_handle(command, &pool).await;
    }else {
        println!("not now");
    }
    
    

    //end of the program
}
