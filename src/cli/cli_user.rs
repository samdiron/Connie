
use std::net::SocketAddr;

use common_lib::{
    path::SQLITEDB_PATH,
    gethostname::gethostname, 
};

use lib_db::{
    database::get_conn,
    user::user_struct::User,
    sqlite::{get_sqlite_conn, sqlite_user::ShortUser},
};

use tcp::client::client::signup_process;

use crate::{cli::Commands, get_new_pass};

pub async fn handle_cli_user(command: Commands) {
    match command {
        Commands::User {
            new,
            signup,
            update,
            host,
            ip,
            port,
            admin,
            name,
            username,
            email,
        } => {
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

            } else if signup.is_some() && signup.unwrap() && port.is_some() && ip.is_some() {
                let mut password = String::new();
                let pool = get_sqlite_conn(&SQLITEDB_PATH.to_string()).await.unwrap();
                println!("you are creating a user for a host");
                get_new_pass(&mut password, &name);
                let user = ShortUser {
                    name,
                    username,
                    email,
                    password  
                };
                let addr = SocketAddr::new(ip.unwrap(), port.unwrap());
                // signup_process(addr, user, &pool).await.unwrap();
            }else if signup.is_some() && signup.unwrap() && port.is_some() && ip.is_some() {
                let mut password = String::new();
                let pool = get_sqlite_conn(&SQLITEDB_PATH.to_string()).await.unwrap();
                println!("you are creating a user for a host");
                get_new_pass(&mut password, &name);
                let user = ShortUser {
                    name,
                    username,
                    email,
                    password  
                };
                let addr = SocketAddr::new(ip.unwrap(), port.unwrap());
                signup_process(addr, user, &pool).await.unwrap();
            }
        } 
        _=> {}
    }
}
