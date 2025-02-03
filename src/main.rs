use std::{io::{stdout, Write}, net::IpAddr, process::exit, str::FromStr};

// use std::fs::remove_file;
use lib_db::{database::get_conn, server::server_struct::get_server};
use rpassword::{prompt_password, read_password};
use tcp::{client::client::client_process, server::listener};

use serde::{Serialize, Deserialize};
use clap::{Parser, Subcommand};





//NOTE: for this progrm to start you have to write your postgres connection url
//like this postgres://db_user:password_for_the_user@ip:port/database_name
//postgres default port is 5432, and ip by default is localhost
//in the /Connie/etc/db_conn; file

#[derive(Debug, Deserialize, Serialize, Parser)]
#[command(version = "0.1beta", about = "a web server in rust for more info visit https://github.com/samdiron/Connie")]
struct Cli {
    #[arg(long, short)]
    bind: Option<String>,

    #[arg(long, short)]
    connect: Option<String>,
    
    #[arg(long, short)]
    name: Option<String>,

    #[arg(long, short)]
    file: Option<String>,

    #[arg(long, short)]
    get: Option<String>,

    #[arg(long, short)]
    post:Option<String>,

    #[arg(long)]
    db: Option<String>,

    #[arg(long, short)]
    host: Option<String>,


    #[arg(long, short)]
    port: Option<u16>,


    #[command(subcommand)]
    config: Option<Config>

}

#[derive(Debug, Deserialize, Serialize,Subcommand)]
enum Config {
     
    SERVER {
        #[arg(long)]
        new: bool,

        #[arg(long)]
        update: bool,

        #[arg(long, short)]
        ip: Option<String>,

        #[arg(long, short)]
        name: String,

    },


    User {
        #[arg(long)]
        new: bool,

        #[arg(long)]
        update: bool,
        
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


#[tokio::main(flavor = "multi_thread")]
async fn main() {
    //start of the program 

    let _cli = Cli::parse();


    let pool =  get_conn().await.unwrap();
    listener::bind(pool).await;
    //end of the program
}
