use serde::{Deserialize, Serialize};
use crate::sha256::digest;
use sqlx::{Error, PgPool, Row};
use uuid::Uuid;

use crate::escape_user_input;

#[derive(Serialize, Deserialize, Clone)]
pub struct Server {
    pub cpid: String,
    pub name: String,
    pub host: String,
    pub pri_ip: String,
    pub pub_ip: String,
    pub memory: i64,
    pub max_conn: i16,
    pub password: String,
}
pub async fn get_server(
    name: String,
    password: String,
    pool: &PgPool,
) -> sqlx::Result<Server, sqlx::Error> {
    let sql = "SELECT * FROM server WHERE name = $1 AND password = $2;";
    let row = sqlx::query(sql)
        .bind(name)
        .bind(password)
        .fetch_one(pool)
        .await?;
    let server = Server {
        pri_ip: row.get("pri_ip"),
        pub_ip: row.get("pub_ip"),
        cpid: row.get("cpid"),
        name: row.get("name"),
        host: row.get("host"),
        memory: row.get("memory"),
        max_conn: row.get("max_conn"),
        password: row.get("password"),
    };
    Ok(server)
}

impl Server {
    pub async fn create(self, pool: &PgPool) -> sqlx::Result<Server, Box<Error>> {
        let password = digest(self.password.clone());
        let cpid = Uuid::new_v4().to_string();

        let sql = format!(
            "INSERT INTO server(cpid, name, host, memory, max_conn, pri_ip, pub_ip, password) 
            VALUES ('{}', '{}', '{}', {}, {}, '{}', '{}', '{}'); ",
            escape_user_input(&cpid),
            escape_user_input(&self.name),
            escape_user_input(&self.host),
            self.memory,
            self.max_conn,
            escape_user_input(&self.pri_ip),
            escape_user_input(&self.pub_ip),
            escape_user_input(&password),
            
        );
        sqlx::query(&sql)
            .execute(pool)
            .await?;
        let server = Server {
            cpid,
            name: self.name,
            host: self.host,
            pri_ip: self.pri_ip,
            pub_ip: self.pub_ip,
            memory: self.memory,
            max_conn: self.max_conn,
            password,
        };

        Ok(server)
    }

    pub async fn update(&self, pool: &PgPool) -> sqlx::Result<(), Box<Error>> {
        let sql = format!(
"UPDATE server SET name = '{}' , memory = {}, max_conn = {} WHERE cpid = '{}' AND password = '{}' ;",
            escape_user_input(&self.name),
            self.memory,
            self.max_conn,
            escape_user_input(&self.cpid),
            escape_user_input(&self.password),
        );
        sqlx::query(&sql)
            .execute(pool)
            .await?;
        Ok(())
    }
}
