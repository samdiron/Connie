use common_lib::gethostname::gethostname;
use sqlx::{PgPool, Row};

pub async fn in_storage(pool: &PgPool) -> u64 {
    let host = gethostname().to_owned();
    let sql = format!("SELECT (size) FROM media WHERE in_host = {:?}", host);
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
