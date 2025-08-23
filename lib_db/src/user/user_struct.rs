use uuid::Uuid;
use sqlx::{PgPool, Result};

use crate::{escape_user_input, hash_passwords};

pub struct User {
    pub cpid: String,
    pub name: String,
    pub username: String,
    pub host: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub async fn create(self, pool: &PgPool) -> Result<User, sqlx::Error> {
        let cpid = Uuid::new_v4().to_string();
        let pass = hash_passwords(self.password);


        let table = r#" "user" "#;
        let sql = format!("
INSERT INTO {table} 
(cpid, name, username, host, email, password)
VALUES ('{}', '{}', '{}', '{}', '{}', '{}');",
            escape_user_input(&cpid),
            escape_user_input(&self.name),
            escape_user_input(&self.username),
            escape_user_input(&self.host),
            escape_user_input(&self.email),
            escape_user_input(&pass)
        );

        let _res = sqlx::query(&sql)
            .execute(pool)
            .await?;
        let new_user = User {
            cpid,
            name: self.name,
            username: self.username,
            host: self.host,
            email: self.email,
            password: pass,
        };
        Ok(new_user)
    }
}
