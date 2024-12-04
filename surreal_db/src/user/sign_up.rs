// use std::hash::Hash;
// Deserialize && token  will be needed later
use crate::db::DB;
use serde::Serialize;
use surrealdb::opt::auth::Scope;

#[derive(Serialize)]
pub struct User {
    pub user_name: String,
    pub name: String,
    pub cpid: String,
    pub pass: String,
}

impl User {
    pub async fn sign_up_N(self) -> surrealdb::Result<String> {
        let jwt = DB
            .signup(Scope {
                namespace: "users",
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

    pub async fn sign_up_A(self) -> surrealdb::Result<String> {
        let jwt = DB
            .signup(Scope {
                namespace: "private_infer",
                database: "admin",
                scope: "admin",
                params: User {
                    user_name: self.user_name,
                    name: self.name,
                    cpid: self.cpid,
                    pass: self.pass,
                },
            })
            .await
            .expect("Error could not get jwt for admin");

        let jwt = jwt.into_insecure_token();
        Ok(jwt)
        //in the feature move clone stream and send the jwt by an tcp/http
    }
}
