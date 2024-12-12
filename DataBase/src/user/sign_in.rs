// Deserialize && token  will be needed later
use crate::db::DB;
use serde::Serialize;
use surrealdb::opt::auth::Scope;
use uuid::Uuid;


#[derive(Serialize)]
pub struct User<> {
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
                    cpid: self.cpid,
                    pass: self.pass,
                },
           })
            .await?;
        let jwt = dbg!(jwt);
        let token = jwt.into_insecure_token();
        Ok(token)
    }
    
    pub async fn sign_up_A(self) -> surrealdb::Result<String> {
        
        let jwt = DB.signin(Scope {
            namespace: "private_infer",
            database: "admin",
            scope: "admin",
            params: User {
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

