use serde::{Deserialize, Serialize};
use sha256::digest;
use sqlx::{Error, PgPool, Row};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Server {
    pub ip: String,
    pub cpid: String,
    pub name: String,
    pub host: String,
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
        ip: row.get("ip"),
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
        let sql = "INSERT INTO server(
        cpid, name, host, memory, max_conn,ip , password) 
        VALUES ($1, $2, $3, $4, $5, $6, $7); ";
        let password = digest(self.password.clone());
        let cpid = Uuid::new_v4().to_string();
        sqlx::query(sql)
            .bind(cpid.clone())
            .bind(&self.name)
            .bind(&self.host)
            .bind(&self.memory)
            .bind(&self.max_conn)
            .bind(&self.ip)
            .bind(password.clone())
            .execute(pool)
            .await?;
        let server = Server {
            ip: self.ip,
            cpid,
            name: self.name,
            host: self.host,
            memory: self.memory,
            max_conn: self.max_conn,
            password,
        };

        Ok(server)
    }

    pub async fn update(&self, pool: &PgPool) -> sqlx::Result<(), Box<Error>> {
        let sql = "UPDATE server SET name = $1 , memory = $2, storage = $3, max_conn = $4 WHERE cpid = $5 AND password = $6;";
        sqlx::query(sql)
            .bind(&self.name)
            .bind(&self.memory)
            .bind(&self.max_conn)
            .bind(&self.cpid)
            .bind(&self.password)
            .execute(pool)
            .await?;
        Ok(())
    }
}
