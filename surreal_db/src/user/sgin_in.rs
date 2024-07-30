//use std::io::Error;

use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::{Root, Scope};
use surrealdb::Surreal;
use tokio;

#[derive(Serialize)]
pub struct User<'a> {
    cpid: &'a str,
    pass: &'a str,
}

impl User<'a> {
    pub async fn sgin_in(&self) -> surrealdb::Result<()> {
        let db: Surreal<Client> = Surreal::new::<Ws>("0.0.0.0:8000").await?;

        db.signin(Root {
            username: "root",
            password: "root",
        })
        .await?;
        db.use_ns("user").use_db("test").await?;
        let jwt = db
            .signin(Scope {
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
