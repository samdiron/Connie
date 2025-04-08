// #![allow(unused_assignments)]
#![allow(unused_variables)]
mod cli;

use std::{
    io::{stdout, Write},
    process::exit
};
use cli::config_handle;
use env_logger;

use crate::cli::Commands;

use lib_start::certs;
use common_lib::rpassword::read_password;
use clap::Parser;




//NOTE: for this progrm to start you have to write your postgres connection url
//like this postgres://db_user:password_for_the_user@ip:port/database_name
//postgres default port is 5432, and ip by default is localhost
//in the /Connie/etc/db_conn; file

#[derive(Debug,Parser)]
#[command(version = "v0.3", about = "a web server in rust for more info visit https://github.com/samdiron/Connie")]
#[command(disable_help_flag = true)]
struct Cli {
    
    #[command(subcommand)]
    config: Option<Commands>,

    #[arg(long, short, default_value="false")]
    tui: Option<bool>,

    #[arg(long, short, default_value="1")]
    verbose: Option<u8>,

    #[arg(long, short, default_value = "false")]
    generate_certs: Option<bool>,

}



pub fn get_pass(password: &mut String, name: &str) {
        print!("enter password for {} : ", name);
        stdout().flush().expect("could not flush");
        let password1 = read_password().unwrap();
        *password = password1;
}


pub fn get_new_pass(password: &mut String, name: &str) {
    for i in 0..2 {
        print!("enter password for {}: ", name);
        stdout().flush().expect("could not flush");
        let password1 = read_password().unwrap();
        print!("confirm password: ");
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
                    .parse_filters("WARN")
                    .parse_filters("ERROR")
                    .parse_filters("INFO")
                    .parse_filters("DEBUG")
                    .init();
            }
            _ => {
            env_logger::Builder::new()
                .parse_filters("trace")
                .init();

            }
        }

    }
    
    
    if let Some(command) = _cli.config {
        config_handle(command).await;
    }else {
        todo!()
    }
    exit(0);
    
    

    //end of the program
}
