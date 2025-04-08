
#![allow(dead_code)]

use std::path::PathBuf;
use std::{io::Result, process::exit};
use std::io::{BufWriter, Read, Write};
use std::fs::File;
use common_lib::{log::error, path::SERVER_IDENT, toml};
use lib_db::server::server_struct::Server;
use serde::{Deserialize, Serialize};

pub const SQLITE: &str = "SQLITE";
pub const POSTGRES: &str = "POSTGRES";

pub const PRI_NET: &str = "local";
pub const ALL_AV_NET: &str = "NET";

#[derive(Serialize, Deserialize)]
pub struct ServerIdent {
    pub default_database: String,
    pub new_users: bool,
    pub default_network: String,
    pub default_port: u16,
    pub default_server: Server,
}

pub async fn get_server_config() -> Result<ServerIdent> {
    let mut f = File::open(SERVER_IDENT)
        .expect("make sure you are runing this command in root status");
    let mut buf = String::new();
    f.read_to_string(&mut buf)
        .expect("could not read SERVER_IDENT");
    let structed: ServerIdent  = toml::from_str(&buf).unwrap();
    Ok(structed)

}


impl ServerIdent{
    pub async fn create_config(s:Self) {
        if s.default_network.as_str() != PRI_NET && s.default_network.as_str() != ALL_AV_NET {
            error!("default_network does not match expected input");
            exit(1)
        };
        if s.default_database.as_str() != SQLITE &&s.default_database.as_str() != POSTGRES {
            error!("default_database does not match expected input");
            exit(1)
        }
        let tomled = toml::to_string(&s);
        
        if tomled.is_ok() {
            let path = PathBuf::from(SERVER_IDENT);
            let toml = tomled.unwrap();
            println!("toml: {}", &toml);
            let f = if !path.exists() {
                File::create_new(path).unwrap()
            } else { File::open(path).unwrap()};
            let mut f = BufWriter::new(f);
            f.write_all(toml.as_bytes())
                .expect(" write make sure you are runing this command in root status");
            f.flush().unwrap();
            drop(s);
        }else {
            error!("could not create server_ident: {:#?}", tomled.unwrap_err());
        }
    }
}


