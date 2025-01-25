use log::info;
use sqlx::{PgPool, Result};
use std::io::Read;

pub async fn migrate(pool: &PgPool) -> Result<(), sqlx::Error> {
    info!("INFO: db migration");
    {
        println!("migrate:1");
        let sql = crate::migrations::server_table::get_sql();
        let _res = sqlx::query(sql.as_str()).execute(pool).await?;
    }
    {
        println!("migrate:2");
        let sql = crate::migrations::user_table::get_sql();
        let _res = sqlx::query(sql.as_str()).execute(pool).await?;
    };

    {
        println!("migrate:3");
        let sql = crate::migrations::admin_table::get_sql();
        let _res = sqlx::query(sql.as_str()).execute(pool).await?;
    }
    {
        println!("migrate:4");
        let sql = crate::migrations::media_table::get_sql();
        let _res = sqlx::query(sql.as_str()).execute(pool).await?;
    }
    {
        println!("migrate:5");
        let sql = crate::migrations::user_av_jwt::get_sql();
        let _res = sqlx::query(sql.as_str()).execute(pool).await?;
    }
    Ok(())
}

pub async fn get_conn() -> Result<PgPool, sqlx::Error> {
    let mut url = String::new();
    let mut file = std::fs::File::open("/Connie/etc/db_conn")
        .expect("error opening connection file from /Connie/etc/db_conn");
    let _res = file
        .read_to_string(&mut url)
        .expect("could not get connection string from file /Connie/database.db/connection");
    println!("PGPOOL conn: {}", url);
    let pool = sqlx::postgres::PgPool::connect(url.as_str()).await?;

    Ok(pool)
}
