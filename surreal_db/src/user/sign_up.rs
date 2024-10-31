use crate::db::DB;
use serde::{Deserialize, Serialize};
use surrealdb::opt::auth::Root;
use surrealdb::opt::auth::Record;
use uuid::Uuid;


#[derive(Serialize,Debug, Deserialize)]
pub struct DUser {
    pub is_admin: bool,
    pub user_name: String,
    pub name: String,
    pub cpid: Uuid,
    pub pass: String,
}


#[derive(Debug, Deserialize, Serialize )]
pub struct User {
    pub user_name: String,
    pub name: String,
    pub cpid: Uuid,
    pub pass: String,
}

impl DUser {
    pub async fn sign_up_admin(self) -> surrealdb::Result<()> {
        //     DB.signin(Root {
        //     username: "root",
        //     password: "root",
        // }).await?;
        DB.use_ns("private_infer").use_db("admin").await?;

        //     let sql = r#"
        // DEFINE ACCESS ADUser_access ON DATABASE TYPE RECORD DURATION 74h
        // SIGNUP ( CREATE user SET name = $name, pass = crypto::argon2::generate($pass),user_name = $user_name, is_admin = $is_admin, cpid = $cpid)
        // "#;
        // DB.query(sql).await?.check()?;
        // println!("sql def access");
        let jwt = DB
            .signup( Record{
                namespace: "private_infer",
                database: "admin",
                access: "ADUser_access",
                params: DUser {
                    is_admin: self.is_admin,
                    user_name: self.user_name.clone(),
                    name: self.name,
                    cpid: self.cpid,
                    pass: self.pass,
                },
            })
            .await.expect("could not create admin");
        let token = jwt.as_insecure_token();
        dbg!(token);

        println!("admin user signup token generated for user:{}",self.user_name);
        Ok(())
    }
}

impl User {
    pub async fn sign_up(self) -> surrealdb::Result<String> {
        DB.signin(Root{
            username: "root",
            password: "root",
        }).await?;
        DB.use_ns("client").use_db("user").await?;


        let sql = r#"
        DEFINE ACCESS User_access ON DATABASE TYPE RECORD DURATION 24h
        SIGNUP ( CREATE user SET name = $name, pass = crypto::argon2::generate($pass),user_name = $user_name, cpid = $cpid)
        "#;
        DB.query(sql).await?.check()?;

        let jwt = DB
            .signup(Record{
                namespace: "client",
                database: "user",
                access: "User_access",
                params: User {
                    user_name: self.user_name,
                    name: self.name,
                    cpid: self.cpid,
                    pass: self.pass,
                },
            })
            .await?;
        let token = jwt.as_insecure_token();
        Ok(token.to_owned())
    }
}
