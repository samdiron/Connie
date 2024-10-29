use std::path::PathBuf;
use once_cell::sync::Lazy;
use surrealdb::engine::remote::ws::{Client, Ws, Wss};
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use lazy_static::lazy_static;
use surrealdb::engine::local::RocksDb;
use directories::BaseDirs;


fn h_path() -> String {
    let mut path :PathBuf = PathBuf::new();
    if let Some(base) = BaseDirs::new() {
        *&mut path = base.home_dir().to_owned();
    }
    let sg = path.to_str().unwrap();
    let string = sg.to_owned();
    let string = format!("{string}/Connie/surreal/Connie.db");
    return string
}

async fn base() -> Surreal<Db>{//surrealdb::api::engine::local::Db> {
    let path = h_path();
    let db = Surreal::new::<RocksDb>(path).await.expect("could not connect to db ");
    return db
}
pub lazy_static! {
    static ref PATH : String = { let path = h_path(); return path};
    static ref DATABASE: Surreal<surrealdb::api::engine::local::Db>  = {
        let bind = base().poll();
        return bind
    }
}

pub static DBASE: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

#[derive()]
pub struct DB<'a> {
    pub addr: &'a str,
    pub remote: bool,
}

impl<'a> DB<'a> {
    pub async fn connect(self) -> surrealdb::Result<()> {
        let ip = format!("{}:8060", self.addr);
        if !self.remote {
            DBASE.connect::<Ws>(ip.as_str()).await?;
            Ok(())
        } else {
            DBASE.connect::<Wss>(ip.as_str()).await?;
            Ok(())
        }
    }
}
