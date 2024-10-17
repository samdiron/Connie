// Deserialize && token  will be needed later
use crate::db::DBASE;
use serde::Serialize;
use surrealdb::opt::auth::Scope;
use uuid::Uuid;

//static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);
//rename cpid to a better thing and add a uuid
#[derive(Serialize)]
pub struct User<'a> {
    pub cpid: Uuid,
    pub pass: &'a str,
}

impl<'a> User<'a> {
    pub async fn login_in(self) -> surrealdb::Result<String> {
        let jwt = DBASE
            .signin(Scope {
                namespace: "user",
                database: "client",
                scope: "user",
                params: User {
                    cpid: self.cpid,
                    pass: self.pass,
                },
            })
            .await?;
        let token = jwt.into_insecure_token();
        Ok(token)
    }
}
