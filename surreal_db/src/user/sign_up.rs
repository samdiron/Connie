// use std::hash::Hash;
// Deserialize && token  will be needed later
use crate::db::DB;
use serde::{Deserialize, Serialize};
use surrealdb::opt::auth::Scope;
use uuid::Uuid;
//static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);
//rename cpid to a better thing and add an uuid
#[derive(Serialize,Debug, Deserialize)]
pub struct DUser {
    pub is_admin: bool,
    pub user_name: String,
    pub name: String,
    pub cpid: Uuid,
    pub pass: String,
}
#[derive(Serialize)]
pub struct User {
    pub user_name: String,
    pub name: String,
    pub cpid: Uuid,
    pub pass: String,
}
impl DUser {
    pub async fn sign_up_admin(self) -> surrealdb::Result<String> {
        let jwt = DB
            .signup(Credentials {
                namespace: "private_infer",
                database: "admin",
                scope: "user",
                params: DUser {
                    is_admin: self.is_admin,
                    user_name: self.user_name,
                    name: self.name,
                    cpid: self.cpid,
                    pass: self.pass,
                },
            })
            .await?;
        let token = jwt.as_insecure_token();
        dbg!(token);
        Ok(())

    }
}

impl User {
    pub async fn sign_up(self) -> surrealdb::Result<String> {
        let jwt = DBASE
            .signup(Scope {
                namespace: "user",
                database: "test",
                scope: "user",
                params: User {
                    user_name: self.user_name,
                    name: self.name,
                    cpid: self.cpid,
                    pass: self.pass,
                },
            })
            .await?;
        let token = jwt.into_insecure_token();
        Ok(token)
    }
}
