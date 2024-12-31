use sqlx::{PgPool, Result};

pub struct User {
    pub cpid: String,
    pub name: String,
    pub username: String,
    pub host: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub async fn create(self, pool: &PgPool) -> Result<(), sqlx::Error> {
        let sql = concat!(
            r#"INSERT INTO "user"#,
            "(cpid, name, username, host, email, host)",
            "VALUES ($1, $2, $3, $4, $5, $6);"
        );

        let _res = sqlx::query(sql)
            .bind(&self.cpid)
            .bind(&self.name)
            .bind(&self.username)
            .bind(&self.host)
            .bind(&self.email)
            .bind(&self.password)
            .execute(pool)
            .await?;
        Ok(())
    }
}
