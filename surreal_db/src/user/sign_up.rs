// use std::hash::Hash;
// Deserialize && token  will be needed later
use crate::db::DB;
use serde::{Deserialize, Serialize};
use surrealdb::opt::auth::Scope;
use uuid::Uuid;


#[derive(Deserialize,Serialize)]
pub struct User {
    pub is_admin: Option<bool>,
    pub user_name: String,
    pub name: String,
    pub cpid: Uuid,
    pub pass: String,
}


impl User {
    pub async fn sign_up_N(self) -> surrealdb::Result<String> {
        let jwt = DB
            .signup(Scope {
                namespace: "user",
                database: "test",
                scope: "user",
                params: User {
                    is_admin: None,
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
        
        let jwt = DB.signin(Scope {
            namespace: "private_infer",
            database: "admin",
            scope: "admin",
            params: User {
                is_admin: Some(true),
                user_name:self.user_name,
                name: self.name,
                cpid: self.cpid,
                pass:self.pass,
            }

        }).await.expect("Error could not get jwt for admin");

        let jwt  = dbg!(jwt);

        let jwt = jwt.into_insecure_token();  
        Ok(jwt)
        //in the feature move clone stream and send the jwt by an tcp/http 
        
    }


}
