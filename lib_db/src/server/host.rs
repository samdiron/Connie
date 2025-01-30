use sqlx::PgPool;
use sqlx::Result;
use sqlx::Row;

pub async fn get_host_ip(host: String, pool: &PgPool) -> Result<String> {
    let sql = r#"
        SELECT ip FROM server WHERE host = $1;
    "#;
    let res = sqlx::query(sql).bind(host).fetch_one(pool).await?;
    let ip = res.get("ip");
    Ok(ip)
}
