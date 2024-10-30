#![allow(clippy::let_and_return)]


use std::path::PathBuf;
use once_cell::sync::Lazy;
use surrealdb::engine::remote::ws::{Client, Ws, Wss};
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use lazy_static::lazy_static;
use surrealdb::engine::local::RocksDb;
use directories::BaseDirs;
use tokio::runtime;

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

async fn base() -> Surreal<Db>{//surrealdb::api::engine::local::Db> {
     let path = h_path();

     let db =  Surreal::new::<RocksDb>(path).await.expect("could not connect to db ");
     db
}


lazy_static! {
    pub static ref PATH : String = { 
        let path = h_path();
        return path
    };
    pub static ref DBASE: Surreal<Db>  = {
        let rt = runtime::Runtime::new().unwrap();
        let bind = rt.block_on(base());
        return bind
    };
}

// pub static DBASE: Lazy<Surreal<Db>> = Lazy::new(Surreal::new::<RocksDb>(PATH));


pub static NTLDB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);


#[derive()]
pub struct NLDB<'a> {
    pub addr: &'a str,
    pub remote: bool,
}

impl<'a> NLDB<'a> {
    pub async fn connect(self) -> surrealdb::Result<()> {
        let ip = format!("{}:8060", self.addr);
        if !self.remote {
            NTLDB.connect::<Ws>(ip.as_str()).await?;
            Ok(())
        } else {
            NTLDB.connect::<Wss>(ip.as_str()).await?;
            Ok(())
        }
    }
}
