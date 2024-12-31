use log::{error, info};
use once_cell::sync::Lazy;
use sqlx::{PgPool, Result};
use std::{io::Read, process::abort};

pub static POOL: Lazy<PgPool> = Lazy::new(|| {
    let pool = get_conn();
    if pool.is_ok() {
        info!("INFO: db pool obtained ");
        let pool = pool.unwrap();
        pool
    } else {
        error!("ERROR: can't get db pool");
        abort();
    }
});
pub async fn migrate(pool: &PgPool) -> Result<(), sqlx::Error> {
    info!("INFO: db migration");
    {
        let sql = crate::migrations::user_table::get_sql();
        let _res = sqlx::query(sql.as_str()).execute(pool).await?;
    };
    {
        let sql = crate::migrations::server_table::get_sql();
        let _res = sqlx::query(sql.as_str()).execute(pool).await?;
    }
    {
        let sql = crate::migrations::media_table::get_sql();
        let _res = sqlx::query(sql.as_str()).execute(pool).await?;
    }
    Ok(())
}

#[tokio::main]
async fn get_conn() -> Result<PgPool, sqlx::Error> {
    let mut url = String::new();
    let mut file = std::fs::File::open("/Connie/etc/db_conn")
        .expect("error opening connection file from /Connie/etc/db_conn");
    let _res = file
        .read_to_string(&mut url)
        .expect("could not get connection string from file /Connie/database.db/connection");
    let pool = sqlx::postgres::PgPool::connect(url.as_str()).await?;

    Ok(pool)
}
