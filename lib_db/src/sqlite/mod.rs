pub mod sqlite_host;

pub mod sqlite_jwt;

pub mod sqlite_user;

pub mod sqlite_media;

use sqlx::{sqlite, Result};
pub async fn get_sqlite_conn(path: PathBuf) -> Result<SqlitePool> {
    let url = format!(":{}:", path.to_str().unwrap());
    let conn = sqlite::SqlitePool::connect(&url).await?;
    Ok(conn)
} 


use std::path::PathBuf;

use common_lib::log::info;
use sqlx::SqlitePool;
pub async fn migration(pool: &SqlitePool) {
    
    info!("migration user;");
    sqlite_user::create_table(pool).await;
    
    info!("migration host;");
    sqlite_host::create_table(pool).await;

    info!("migration jwt;");
    sqlite_jwt::create_table(pool).await;

    info!("migration media;");
    sqlite_media::create_table(pool).await;

}
