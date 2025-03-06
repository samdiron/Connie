use sqlx::{PgPool, Row};

pub async fn in_storage(pool: &PgPool, host_cpid: &String) -> u64 {
    let sql = format!("SELECT (size) FROM media WHERE in_host = {:?}", host_cpid);
    let _res = sqlx::query(&sql).fetch_all(pool).await;
    if _res.is_ok() {
        let mut  in_storage = 0i64;
        for row in _res.unwrap() {
            let s: i64 = row.get("size");
            in_storage+=s;
        }
        return in_storage as u64;
    }

    return 0
}
