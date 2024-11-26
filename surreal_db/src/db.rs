#![allow(clippy::let_and_return)]
#![allow(clippy::collapsible_else_if)]
use crate::sql_fnc::DEFINE::{define_scope_admin, define_scope_user};
use directories::BaseDirs;
use once_cell::sync::Lazy;
use std::path::PathBuf;
use std::process::exit;
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::engine::remote::ws::{Client, Ws, Wss};
use surrealdb::Surreal;

fn h_path() -> String {
    let mut path = PathBuf::new(); // = PathBuf::new();
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

pub async fn first_time_db_def() -> surrealdb::Result<()> {
    let _admin = define_scope_admin().await?;
    let _user = define_scope_user().await?;
    Ok(())
}

#[derive()]
pub struct DBC {
    pub addr: Option<String>,
    pub remote: bool,
    pub lm: bool,
}

impl DBC {
    pub async fn connect(self) {
        if self.lm {
            let path = h_path();
            let rocks = format!("rocksdb://{path}");
            DB.connect::<RocksDb>(rocks)
                .await
                .expect("could not connect to local db");
            println!("lmdb connected")
        } else {
            if let Some(addr) = self.addr {
                let ip = format!("{}:8060", addr);

                if !self.remote {
                    WDB.connect::<Ws>(ip.as_str())
                        .await
                        .expect("could not connect to ws");
                } else {
                    WDB.connect::<Wss>(ip.as_str())
                        .await
                        .expect("could not connect to wss");
                }
            } else {
                exit(8000)
            }
        }
    }
}
