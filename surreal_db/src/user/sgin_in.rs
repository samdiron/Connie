//use std::io::Error;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws, Wss};
use surrealdb::opt::auth::Scope;
use surrealdb::Surreal;
use crate::db;
static DBASE: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);
#[derive(Serialize)]
pub struct User<'a> {
    pub cpid: &'a str,
    pub pass: &'a str,
}

impl <'a>User<'a> {
    pub async fn login_in(&self, &db: &db::DB) -> surrealdb::Result<()> {
        let db = if db.isremote.unwrap() == true {
            let addr = db.addr.unwrap();
            let db = DBASE.connect::<Wss>(addr).await?;
            db
        } else {
            let addr = db.addr.unwrap();
            let db = DBASE.connect::<Ws>(addr).await?;
            db
        };
        let jwt = db.signin(Scope {
                namespace: "user",
                database: "test",
                scope: "user",
                params: User {
                    cpid: &self.cpid,
                    pass: &self.pass,
                },
            })
            .await?;
        let token = jwt.as_insecure_token();
        Ok(())
    }
}
