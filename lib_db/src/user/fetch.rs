use common_lib::sha256::digest;
use sqlx::Row;
use sqlx::PgPool;

use crate::escape_user_input;
use crate::user::user_struct::User;




pub async fn fetch_wcpid(
    cpid: String,
    _password: String,
    pool: &PgPool,
) -> sqlx::Result<User, sqlx::Error> {
    let table = r#" "user" "#;
    let sql = format!("
SELECT * 
FROM {table} 
WHERE cpid = '{}' AND password = '{}';",
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
    let password = digest(_password);
    let sql = format!("
SELECT * 
FROM {table} 
WHERE name = '{}' AND password = '{}' ;",
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
