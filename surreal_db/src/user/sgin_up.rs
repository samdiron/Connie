use once_cell::sync::Lazy;
// Deserialize && token  will be needed later
use serde::{Serialize};
use surrealdb::engine::remote::ws::{Client, Wss};
use surrealdb::opt::auth::Scope;
use crate::db::DBASE;

//static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);
//rename cpid to a better thing and add a uuid
#[derive(Serialize)]
pub struct DUser<'a> {
    pub is_admin: &'a bool,
    pub user_name: &'a str,
    pub name: &'a str,
    pub cpid: &'a str,
    pub pass: &'a str,
}
pub struct User<'a> {
    pub user_name: &'a str,
    pub name: &'a str,
    pub cpid: &'a str,
    pub pass: &'a str,
}
impl <'a>DUser<'a> {
    pub async fn sign_up_admin(self) -> surrealdb::Result<(&'a str)> {


        let jwt = DBASE.signup(Scope {
            namespace: "admin",
            database: "private_infer",
            scope: "user",
            params: DUser {
                is_admin: self.is_aadmin,
                user_name: self.user_name,
                name: self.name,
                cpid: self.cpid,
                pass: self.pass,
            },
        })
            .await?;
        let token = jwt.as_insecure_token();
        Ok(token)
    }

}

impl <'a>User<'a> {
    pub async fn sign_up(self) -> surrealdb::Result<(&'a str)> {


        let jwt = DBASE.signup(Scope {
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
        Ok(token)
    }
}
