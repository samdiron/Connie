pub(crate) mod cli_db;
pub(crate) mod cli_dev;
#[allow(unused_assignments)]
pub(crate) mod cli_bind;
pub(crate) mod cli_user;
pub(crate) mod cli_server;
pub(crate) mod cli_request;

use std::{
    net::IpAddr,
    path::PathBuf,
};

use clap::Subcommand;
use cli_db::handle_cli_db;
use cli_user::handle_cli_user;
use cli_bind::handle_cli_bind;
use cli_server::handle_cli_server;
use cli_request::handle_cli_request;

#[allow(non_snake_case)]
#[derive(Debug,Subcommand)]
pub enum Commands {

    DEV {
        #[arg(long,short)]
        tls_config: Option<bool>,
        #[arg(long,short)]
        env_logger: Option<bool>,
    },
     
    BIND {

        #[arg(long, short)]
        admin_port: u16,

        #[arg(long, short)]
        default: Option<bool>,

        #[arg(long, short)]
        ip: Option<String>,

        #[arg(long, short)]
        new_users: Option<bool>,

        #[arg(long, short)]
        users: Option<u64>,
        
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

        #[arg(long, short)]
        Ip: Option<IpAddr>,
        
        #[arg(long)]
        Domain: Option<String>,

        #[arg(long)]
        Port: Option<u16>,

        #[arg(long, short)]
        host: Option<String>,

        #[arg(long, short)]
        server_name: Option<String>,
        
        #[arg(long, short)]
        fetch_files: Option<bool>,

        #[arg(long, short)]
        login: Option<bool>,

        #[arg(long, short)]
        all: Option<bool>,

        #[arg(short, long)]
        db: Option<PathBuf>,

        #[arg(long, short)]
        get: Option<PathBuf>,
        
        #[arg(long, short)]
        post: Option<PathBuf>,

        #[arg(long)]
        Delete: Option<bool>,

        #[arg(long, short, default_value="false")]
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
        connection: Option<String>,

        #[arg(long, short)]
        path: Option<PathBuf>,

        #[arg(long, short)]
        sqlite_migrations: Option<bool>,

        
        #[arg(long, short)]
        postgres_migrations: Option<bool>,



    },


    User {

        #[arg(long, help="this flag creates a user on the postgres database it only works if ")]
        new: Option<bool>,

        #[arg(long)]
        update: Option<bool>,
        
        #[arg(long)]
        host: Option<String>,

        #[arg(long, help="this flag is for the host ip that you want to signun to ")]
        ip: Option<IpAddr>,

        #[arg(long)]
        port: Option<u16>,
        
        #[arg(long, short, help="this flag is for signun to a remote machine")]
        signup: Option<bool>,
        
        #[arg(long, short,)]
        db: Option<PathBuf>,

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




pub async fn config_handle(command: Commands ) {
    match command {
        Commands::User{ .. } => {
            handle_cli_user(command).await
        } 
        Commands::SERVER{ .. }  => {
            handle_cli_server(command).await
        }
        Commands::BIND{ .. }  => {
            handle_cli_bind(command).await
        }
        Commands::DB { .. } => {
            handle_cli_db(command).await
        }
        Commands::REQUEST { .. } => {
            handle_cli_request(command).await
        }
        _ => {}
    }
}

