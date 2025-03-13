
use common_lib::log::debug;
use sha256::digest;
use sqlx::{PgPool, Result, Row};
use uuid::Uuid;
use crate::fncs::escape_user_input;

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
    let clean_paswd = escape_user_input(&paswd);
    let cleen_cpid = escape_user_input(&cpid);
    let sql = format!(
        "SELECT count(1) FROM {} WHERE cpid = '{}' AND password = '{}' ;",
        table,
        cleen_cpid,
        clean_paswd
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
    let clean_paswd = escape_user_input(&paswd);
    let cleen_name = escape_user_input(&name);
    let sql = format!(
        "SELECT count(1) FROM {} WHERE name = '{}' AND password = '{}' ;",
        table,
        cleen_name,
        clean_paswd,
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
    let cpid = escape_user_input(&cpid);
    let _password = escape_user_input(&_password);
    let sql = format!(r#"SELECT * FROM "user" WHERE cpid = '{}' AND password = '{}';"#, cpid , _password);
    debug!("SERVER_DB: {}",&sql);
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
    let name = escape_user_input(&name);
    let password = sha256::digest(_password);
    let sql = format!(
        "SELECT * FROM {table} WHERE name = '{}' AND password = '{}';",
        name,
        password
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
        let name = escape_user_input(&self.name);
        let username = escape_user_input(&self.username);
        let host = escape_user_input(&self.host);
        let email = escape_user_input(&self.email);

        let pass = digest(self.password);
        let cpid = Uuid::new_v4().to_string();
        let sql = format!(
            r#"INSERT INTO "user"
            (cpid, name, username, host, email, password)
            VALUES ('{}', '{}', '{}', '{}', '{}', '{}');"#,
            &cpid,
            &name,
            &username,
            &host,
            &email,
            &pass
        );
        let _res = sqlx::query(&sql)
            .execute(pool)
            .await?;
        let new_user = User {
            cpid,
            name,
            username,
            host,
            email,
            password: pass,
        };
        Ok(new_user)
    }
}
