use sqlx::{sqlite, Result};
pub mod sqlite_host;

pub mod sqlite_jwt;

pub mod sqlite_user;

pub mod sqlite_media;

pub async fn get_sqlite_conn(path: &String) -> Result<SqlitePool> {
    let conn = sqlite::SqlitePool::connect(path).await?;
    debug!("SQLITE Connection Established");
    Ok(conn)
} 


use std::path::PathBuf;

use common_lib::log::{debug, info};
use sqlx::SqlitePool;
pub async fn migration(pool: &SqlitePool) -> Result<()> {
    
    info!("migration user;");
    sqlite_user::create_table(pool).await?;
    
    info!("migration host;");
    sqlite_host::create_table(pool).await?;

    info!("migration jwt;");
    sqlite_jwt::create_table(pool).await?;

    info!("migration media;");
    sqlite_media::create_table(pool).await?;

    Ok(())

}
