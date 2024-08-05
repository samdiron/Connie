use once_cell::sync::Lazy;
// Deserialize && token  will be needed later
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Wss};
use surrealdb::opt::auth::Scope;
//use surrealdb::Surreal;
use crate::db::DBASE;
use crate::db::DB;

//static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);
//rename cpid to a better thing and add a uuid
#[derive(Serialize)]
pub struct User<'a> {
    pub cpid: &'a str,
    pub pass: &'a str,
}

impl <'a>User<'a> {
    pub async fn login_in(self) -> surrealdb::Result<()> {


        let jwt = DBASE.signin(Scope {
                namespace: "user",
                database: "test",
                scope: "user",
                params: User {
                    cpid: self.cpid,
                    pass: self.pass,
                },
            })
            .await?;
        let token = jwt.as_insecure_token();
        Ok(())
    }
}
