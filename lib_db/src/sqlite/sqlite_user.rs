use common_lib::bincode;
use common_lib::log::{debug, error};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use sqlx::{Result, SqlitePool};

use crate::escape_user_input;

#[derive(Serialize, Deserialize)]
pub struct ShortUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub username: String,
}
impl ShortUser {
    
    pub fn sz(self) -> Result<Vec<u8>, bincode::Error> {
        let res: Vec<u8> = bincode::serialize(&self)?;
        drop(self);
        Ok(res)
    }
    
    pub fn dz(buf: Vec<u8>) -> Result<Self, bincode::Error> {
        let res: Self = bincode::deserialize(&buf)?;
        drop(buf);
        Ok(res) 
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct SqliteUser {
    pub name: String,
    pub host: String,
    pub cpid: String,
    pub email: String,
    pub usrname: String,
}

const SQL: &str = "
CREATE TABLE user(
    name TEXT,
    host TEXT,
    cpid TEXT,
    email TEXT,
    usrname TEXT
);";



pub(crate) async fn check_server_users_num(
    server_cpid: &String,
    pool: &SqlitePool
) -> Result<u64> {
    let sql = format!("
SELECT count(*) 
FROM user 
where host = '{}' ;
",
    escape_user_input(server_cpid));
    let res = sqlx::query(&sql)
        .fetch_one(pool)
        .await?;
    let users: i64 = res.get(0usize);

    Ok(users as u64)
}


pub(in crate::sqlite) async fn create_table(pool: &SqlitePool) -> Result<()>{
    debug!("SQLITE: {SQL}");
    sqlx::query(SQL).execute(pool).await.unwrap();
    Ok(())
}


pub async fn fetch_sqlite_user_with_server_cpid(
    username: &String,
    cpid: &String,
    pool: &SqlitePool
) -> Result<SqliteUser> {
    let sql = format!("
SELECT * 
from user 
WHERE usrname = '{}' AND host = '{}' ;",
    escape_user_input(username),
    escape_user_input(cpid),
    );
    let _res = sqlx::query(&sql).fetch_one(pool).await;
    drop(sql);
    if _res.is_ok() {
        let res = _res.unwrap();
        let name: String = res.get("name");
        let host: String = res.get("host");
        let cpid: String = res.get("cpid");
        let email: String = res.get("email");
        let usrname: String = res.get("usrname");
        let user = SqliteUser {
            name,
            host,
            cpid,
            email,
            usrname,
        };
        return Ok(user);
    }else {
        error!("error while trying to fetch user");
        return Err(_res.err().unwrap())
    }
}


pub async fn fetch_sqlite_user(
    username: &String,
    password: &String,
    pool: &SqlitePool
) -> Result<SqliteUser> {
    let sql = format!("
SELECT * 
from user 
WHERE usrname = '{}' AND paswd = '{}';",
    escape_user_input(username),
    escape_user_input(password),
    
    );
    let _res = sqlx::query(&sql).fetch_one(pool).await;
    drop(sql);
    if _res.is_ok() {
        let res = _res.unwrap();
        let name: String = res.get("name");
        let host: String = res.get("host");
        let cpid: String = res.get("cpid");
        let email: String = res.get("email");
        let usrname: String = res.get("usrname");
        let user = SqliteUser {
            name,
            host,
            cpid,
            email,
            usrname,
        };
        return Ok(user);
    }else {
        error!("error while trying to fetch user");
        return Err(_res.err().unwrap())
    }
}


impl SqliteUser {
    pub async fn add_user(
        s: Self,
        pool: &SqlitePool
    ) -> Result<()> {
        let sql = format!(
            "INSERT INTO user(name, host, cpid, email, usrname) VALUES ('{}','{}','{}','{}','{}'); ",
                s.name,
                s.host,
                s.cpid,
                s.email,
                s.usrname,
        );
        sqlx::query(&sql).execute(pool).await?;
        Ok(())
    }


    pub fn sz(self) -> Result<Vec<u8>, bincode::Error> {
        let res: Vec<u8> = bincode::serialize(&self)?;
        drop(self);
        Ok(res)
    }
    
    pub fn dz(buf: Vec<u8>) -> Result<Self, bincode::Error> {
        let res: Self = bincode::deserialize(&buf)?;
        drop(buf);
        Ok(res) 
    }
}
