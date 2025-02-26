
pub use common_lib::path::DB_CONN;
use common_lib::log::info;
use sqlx::{PgPool, Result};
use std::io::Read;

pub async fn migrate(pool: &PgPool) -> Result<(), sqlx::Error> {
    info!("INFO: db migration");
    {
        let sql = crate::migrations::server_table::get_sql();
        println!("1:migrate:{sql}");
        let _res = sqlx::query(sql.as_str()).execute(pool).await?;
    }
    {
        let sql = crate::migrations::user_table::get_sql();
        println!("2:migrate:{sql}");
        let _res = sqlx::query(sql.as_str()).execute(pool).await?;
    };

    {
        let sql = crate::migrations::admin_table::get_sql();
        println!("3:migrate:{sql}");
        let _res = sqlx::query(sql.as_str()).execute(pool).await?;
    }
    {
        let sql = crate::migrations::media_table::get_sql();
        println!("4:migrate:{sql}");
        let _res = sqlx::query(sql.as_str()).execute(pool).await?;
    }
    {
        let sql = crate::migrations::user_av_jwt::get_sql();
        println!("5:migrate:{sql}");
        let _res = sqlx::query(sql.as_str()).execute(pool).await?;
    }
    Ok(())
}
pub fn check_conn() -> u8{
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let is_pool = get_conn().await;
        if is_pool.is_ok() {
            return 0
        }else {
            return 1
        }

    })
}
pub async fn get_conn() -> Result<PgPool, sqlx::Error> {
    let mut url = String::new();
    let mut file = std::fs::File::open(DB_CONN)
        .expect("error opening connection file from /Connie/etc/db_conn");
    let _res = file
        .read_to_string(&mut url)
        .expect("could not get connection string from file /Connie/database.db/connection");
    info!("DATABASE Connection Established");
    let pool = sqlx::postgres::PgPool::connect(url.as_str()).await?;

    Ok(pool)
}
