use sha256::digest;
use sqlx::{PgPool, Result, Row};
use uuid::Uuid;

pub struct User {
    pub cpid: String,
    pub name: String,
    pub username: String,
    pub host: String,
    pub email: String,
    pub password: String,
}

pub async fn fetch(
    cpid: String,
    password: String,
    pool: &PgPool,
) -> sqlx::Result<User, sqlx::Error> {
    let sql = r#"SELECT * FROM "user" WHERE cpid = $1 AND password = $2;"#;
    let row = sqlx::query(sql)
        .bind(cpid)
        .bind(password)
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
        let sql = concat!(
            r#"INSERT INTO "user"
            (cpid, name, username, host, email, password)
            VALUES ($1, $2, $3, $4, $5, $6);"#
        );
        let cpid = Uuid::new_v4().to_string();
        let pass = digest(self.password);
        let _res = sqlx::query(sql)
            .bind(cpid.clone())
            .bind(&self.name)
            .bind(&self.username)
            .bind(&self.host)
            .bind(&self.email)
            .bind(pass.clone())
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
