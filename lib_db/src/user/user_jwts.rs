use sqlx::Result;
use jsonwebtoken::get_current_timestamp;
use sqlx::PgPool;
use sqlx::Row;


pub async fn get_jwt(host: String, cpid: String, pool: &PgPool) -> Result<String> {
    let sql = r#"
       SELECT (jwt) FROM user_jwt WHERE $1 = host AND $2 < date AND $3 = cpid;
    "#;
    let now = get_current_timestamp();
    
    let row = sqlx::query(sql)
        .bind(host)
        .bind(now as i64 )
        .bind(cpid)
        .fetch_one(pool).await?;
    let jwt: String = row.get("jwt");
    Ok(jwt)
}

pub async fn check_exp_jwt(pool: &PgPool) -> Result<u64> {
    let now = get_current_timestamp();

    let sql = r#"
        DELETE * FROM user_jwt WHERE $1 > date;
    "#;

    let rows = sqlx::query(sql)
        .bind(now as i64)
        .execute(pool).await?;
    let num_rows = rows.rows_affected();
    Ok(num_rows)
}


pub async fn add_jwt(
    jwt: String,
    host: String,
    cpid: String,
    pool: &PgPool
) -> Result<u64> {
    let sql = r#"
        INSERT INTO user_jwt(host, jwt, date, cpid) VALUES($1, $2, $3, $4);
    "#;
    let now = get_current_timestamp();

    let _res = sqlx::query(sql)
        .bind(host)
        .bind(jwt)
        .bind(now as i64)
        .bind(cpid)
        .execute(pool).await?;
    let state = _res.rows_affected();
    Ok(state)
}
