#![allow(clippy::let_and_return)]
#![allow(clippy::collapsible_else_if)]

use std::path::PathBuf;
use std::process::exit;
use once_cell::sync::Lazy;
use surrealdb::engine::remote::ws::{Client, Ws, Wss};
use surrealdb::Surreal;
use surrealdb::engine::local::{Db, RocksDb};
use directories::BaseDirs;



fn h_path() -> String {
    let mut path= PathBuf::new();// = PathBuf::new();
    if let Some(base) = BaseDirs::new() {
        path = base.home_dir().to_owned();
    }
    let sg = path.to_str().unwrap();
    let string = sg.to_owned();
    let string = format!("{string}/Connie/surreal/Connie.db");
    string
}

// pub static DBASE: Lazy<Surreal<Db>> = Lazy::new(Surreal::new::<RocksDb>(PATH));


pub static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);
pub static WDB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

#[derive()]
pub struct DBC {
    pub addr: Option<String>,
    pub remote: bool,
    pub lm: bool 
}

impl DBC {
    pub async fn connect(self)  {
        if self.lm {
            let path = h_path();
            let rocks = format!("rocksdb://{path}");
            DB.connect::<RocksDb>(rocks).await.expect("could not connect to local db");
            println!("lmdb connected")
        }else {
            
            if let Some(addr) = self.addr {
                let ip = format!("{}:8060", addr);

                if !self.remote {
                    WDB.connect::<Ws>(ip.as_str()).await.expect("could not connect to ws");

                } else {
                    WDB.connect::<Wss>(ip.as_str()).await.expect("could not connect to wss");

                }
            }
            else {
               exit(8000)
            }
        }
    }
}
