
#![allow(unused_assignments)]
#![allow(unused_variables)]

use std::{fs::remove_file, io::{stdout, Write}, net::IpAddr, path::PathBuf, process::exit};

use common_lib::gethostname::gethostname;
use lib_db::{
    database::{get_conn, DB_CONN},
    server::{host::get_host_info, server_struct::Server},
    user::user_struct::{fetch, User}
};
use tcp::{client::client::client_process, consts::{IP, PORT, USE_IP, USE_PORT}, server::listener::bind, types::{POST, RQM}};
use common_lib::rpassword::read_password;
use common_lib::sysinfo;
use serde::{Deserialize, Serialize};
use clap::{command, Parser, Subcommand};
use tokio::{fs::File, io::AsyncWriteExt};




//NOTE: for this progrm to start you have to write your postgres connection url
//like this postgres://db_user:password_for_the_user@ip:port/database_name
//postgres default port is 5432, and ip by default is localhost
//in the /Connie/etc/db_conn; file

#[derive(Debug, Deserialize, Serialize, Parser)]
#[command(version = "0.2beta", about = "a web server in rust for more info visit https://github.com/samdiron/Connie")]
#[command(disable_help_flag = true)]
struct Cli {
    
    #[arg(long)]
    db: Option<String>,

    #[arg(long, short, default_value="false")]
    tui: Option<bool>,

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


    REQUEST {

        
        #[arg(long, short)]
        user: String,
        
        #[arg(long, short)]
        ip: Option<IpAddr>,
        
        #[arg(long)]
        port: Option<u16>,

        #[arg(long, short)]
        host: Option<String>,

        #[arg(long, short)]
        get: Option<PathBuf>,
        
        #[arg(long, short)]
        post: Option<PathBuf>,



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
        Commands::User { new, update, host, admin, name, username, email } => {
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

            }
        } 
        Commands::SERVER { new, update, ip, name, host, max_conn } => {
            let pool =  get_conn().await.unwrap();
            let pool = &pool;
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
                    let ip = if ip.is_none() {
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
                    
                    println!("server has been create\n will exit now ");
                    exit(0)
                    
                }  
            }
        }
        Commands::BIND { ip, secret, port, server } => {
            use common_lib::env_logger;
            env_logger::init();
            let pool =  get_conn().await.unwrap();
            let pool = &pool;
            let mut passwd = String::new();
            get_pass(&mut passwd, server.as_str());

            let _res = get_host_info(server, passwd, pool).await;
            if _res.is_err() {
                panic!("not a valid server");
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
                if migrations == true {
                    lib_db::database::migrate(pool).await.unwrap();
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
        Commands::REQUEST { ip, port, host, get, post, user} => {
            let _pool = get_conn().await.unwrap();
            let pool = &_pool;
            let mut passwd = String::new();
            get_pass(&mut passwd, user.as_str());
            let usr = fetch(user, passwd, pool).await.expect("could not fetch that user");
            println!("user cpid: {} , name: {}", usr.cpid, usr.username);
            if host.is_some() { 
                let host = host.unwrap();
                println!("creating a checksum this will take a moment");
                let request: RQM = 
                    if post.is_some() {
                        let r = RQM::create(
                            post.unwrap(),
                            POST.to_string(),
                            usr.cpid.clone()
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
    





    
    if let Some(command) = _cli.config {
        config_handle(command).await;
    }else {
        println!("not now");
    }
    
    

    //end of the program
}
