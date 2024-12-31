use sqlx::{Error, PgPool};

pub struct Server {
    pub cpid: String,
    pub name: String,
    pub host: String,
    pub memory: i64,
    pub storage: i64,
    pub max_conn: i64,
    pub password: String,
}

impl Server {
    pub async fn create(&self, pool: &PgPool) -> sqlx::Result<(), Box<Error>> {
        let sql = "INSERT INTO server(cpid, name, host, memory, storage, max_conn, password) VALUES ($1, $2, $3, $4, $5, $6, $7); ";
        sqlx::query(sql)
            .bind(&self.cpid)
            .bind(&self.name)
            .bind(&self.host)
            .bind(&self.memory)
            .bind(&self.storage)
            .bind(&self.max_conn)
            .bind(&self.password)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn update(&self, pool: &PgPool) -> sqlx::Result<(), Box<Error>> {
        let sql = "UPDATE server SET name = $1 , memory = $2, storage = $3, max_conn = $4 WHERE cpid = $5 AND password = $6;";
        sqlx::query(sql)
            .bind(&self.name)
            .bind(&self.memory)
            .bind(&self.storage)
            .bind(&self.max_conn)
            .bind(&self.cpid)
            .bind(&self.password)
            .execute(pool)
            .await?;
        Ok(())
    }
}
