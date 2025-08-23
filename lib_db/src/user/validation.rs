use sqlx::PgPool;
use sqlx::Row;


use crate::escape_user_input;
use crate::user::user_struct::User;


pub async fn check_if_user_same_input(
    user: &User,
    pool: &PgPool
) -> sqlx::Result<bool> {
    let table = r#" "table" "#;
    let sql = format!("
SELECT count(1)
FROM {table}
WHERE name = '{}' AND 
username = '{}' AND 
email = '{}' AND
host = '{}' ;
",
    escape_user_input(&user.name),
    escape_user_input(&user.username),
    escape_user_input(&user.email),
    escape_user_input(&user.host),
    );
    let res = sqlx::query(&sql)
        .fetch_one(pool)
        .await?;
    let num: i64 = res.get("count");
    let status = if num >= 1 {
        true
    } else {
        false
    };

    Ok(status)
}

pub async fn validate_claim_wcpid(
    cpid: &String,
    paswd: &String,
    host_cpid: &String,
    pool: &PgPool
) -> sqlx::Result<bool> {
    let table = r#" "user" "#;
    let sql = format!(
"SELECT count(1) 
FROM {table} 
WHERE cpid = '{}' AND password = '{}' AND host = '{}' ;",
        escape_user_input(cpid),
        escape_user_input(paswd),
        escape_user_input(host_cpid)
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
"SELECT count(1) 
FROM {table} 
WHERE name = '{}' AND password = '{}' ;",
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
