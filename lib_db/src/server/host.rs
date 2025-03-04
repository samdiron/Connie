use sha256::digest;
use sqlx::PgPool;
use sqlx::Result;
use sqlx::Row;

use super::server_struct::Server;


/// gets the host info used for bind --server 
pub async fn get_host_info(
    name: &String,
    password: &String,
    pool: &PgPool
) -> Result<Server> {
    let sql = r#" SELECT * FROM server WHERE name = $1 AND password = $2;"#;
    let password_hash = digest(password);
    let row = sqlx::query(sql)
    .bind(name)
    .bind(password_hash)
    .fetch_one(pool)
    .await?;
    
    let s = Server{
        ip: row.get("ip"),
        cpid: row.get("cpid"),
        name: row.get("name"),
        host: row.get("host"),
        memory: row.get("memory"),
        max_conn: row.get("max_conn"),
        password: row.get("password")

    };
    Ok(s)

}





pub async fn get_host_ip(host: String, pool: &PgPool) -> Result<String> {
    let sql = r#"
        SELECT ip FROM server WHERE host = $1;
    "#;
    let res = sqlx::query(sql).bind(host).fetch_one(pool).await?;
    let ip = res.get("ip");
    Ok(ip)
}
