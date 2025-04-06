use std::{path::PathBuf, str::FromStr};

use sqlx::{PgPool, Row};


pub async fn in_storage_files(pool: &PgPool, host_cpid: &String) -> Vec<PathBuf> {
    let sql = format!(
        "SELECT path FROM media WHERE in_host = '{}';",
        host_cpid
    );
    let _res = sqlx::query(&sql).fetch_all(pool).await;
    let mut files: Vec<PathBuf> = vec![];
    if _res.is_ok() {
        for row in _res.unwrap() {
            let s: String = row.get("path");
            let path = PathBuf::from_str(&s).unwrap();
            files.push(path);
        }
    }
    return files;
}

pub async fn in_storage_size(pool: &PgPool, host_cpid: &String) -> u64 {
    let sql = format!(
        "SELECT SUM(size) FROM media WHERE in_host = '{}' ; ",
        host_cpid
    );
    let _res = sqlx::query(&sql).fetch_one(pool).await;
    if _res.is_ok() {
            let in_storage: i64;
            let bind = _res.unwrap().get("sum");
            in_storage = bind;

            return in_storage as u64;    
    }
    return 0
}
