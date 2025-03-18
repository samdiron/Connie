use sha256::digest;
use sqlx::{PgPool, Result, Row};
use uuid::Uuid;

use crate::escape_user_input;

pub struct User {
    pub cpid: String,
    pub name: String,
    pub username: String,
    pub host: String,
    pub email: String,
    pub password: String,
}
pub async fn validate_claim_wcpid(
    cpid: String,
    paswd: String,
    pool: &PgPool
) -> sqlx::Result<bool> {
    let table = r#" "user" "#;
    let sql = format!(
        "SELECT count(1) FROM {table} WHERE cpid = '{}' AND password = '{}' ;",
        escape_user_input(&cpid),
        escape_user_input(&paswd)
    );
    let _count = sqlx::query(&sql).fetch_one(pool).await?;
    let count: i64 = _count.get("count");
    if count == 1 {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn validate_claim(
    name: String,
    paswd: String,
    pool: &PgPool
) -> sqlx::Result<bool> {
    let table = r#" "user" "#;
    let sql = format!(
        "SELECT count(1) FROM {} WHERE name = '{}' AND password = '{}' ;",
        table,
        escape_user_input(&name),
        escape_user_input(&paswd),
    );
    let _count = sqlx::query(&sql).fetch_one(pool).await;
    if _count.is_err() {Ok(false)}else {
    let count: i64 = _count.unwrap().get("count");
    if count == 1 {
        Ok(true)
    } else {
        Ok(false)
    }
    }
}

pub async fn fetch_wcpid(
    cpid: String,
    _password: String,
    pool: &PgPool,
) -> sqlx::Result<User, sqlx::Error> {
    let table = r#" "user" "#;
    let sql = format!(
        "SELECT * FROM {table} WHERE cpid = '{}' AND password = '{}';",
        escape_user_input(&cpid),
        escape_user_input(&_password)
    );
    let row = sqlx::query(&sql)
        .fetch_one(pool)
        .await?;
    let user = User {
        cpid: row.get("cpid"),
        name: row.get("name"),
        username: row.get("username"),
        host: row.get("host"),
        email: row.get("email"),
        password: row.get("password"),
    };
    Ok(user)
}



pub async fn fetch(
    name: String,
    _password: String,
    pool: &PgPool,
) -> sqlx::Result<User, sqlx::Error> {
    let table = r#" "user" "#;
    let password = sha256::digest(_password);
    let sql = format!(
        "SELECT * FROM {table} WHERE name = '{}' AND password = '{}' ;",
        escape_user_input(&name),
        escape_user_input(&password)
    );
    let row = sqlx::query(&sql)
        .fetch_one(pool)
        .await?;
    let user = User {
        cpid: row.get("cpid"),
        name: row.get("name"),
        username: row.get("username"),
        host: row.get("host"),
        email: row.get("email"),
        password: row.get("password"),
    };
    Ok(user)
}

impl User {
    pub async fn create(self, pool: &PgPool) -> Result<User, sqlx::Error> {
        let cpid = Uuid::new_v4().to_string();
        let pass = digest(self.password);


        let table = r#" "user" "#;
        let sql = format!(
            "INSERT INTO {table}
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
