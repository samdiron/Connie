use common_lib::log::debug;
use serde::{Deserialize, Serialize};
use sqlx::{Result, Row, SqlitePool};



#[derive(Serialize, Deserialize, Clone)]
pub struct SqliteMedia {
    pub name: String,
    pub cpid: String,
    ///note host need the cpid of the host
    pub host: String,
    pub path: String,
    pub type_: String,
    pub checksum: String,
    pub size: i64,
    pub date: i64
}


const SQL: &str = "
CREATE TABLE media(
    name TEXT,
    cpid TEXT,
    host TEXT,
    path TEXT,
    type TEXT,
    checksum TEXT,
    size INT,
    date INT
);";

pub(in crate::sqlite) async fn create_table(pool: &SqlitePool) -> Result<()>{
    debug!("SQLITE: {SQL}");
    sqlx::query(SQL).execute(pool).await?;
    Ok(())
}


pub async fn fetch_all_media_from_host(
    host: &String,
    user: &String,
    pool: &SqlitePool
) -> Result<Vec<SqliteMedia>> {
    let sql = format!("SELECT * FROM media WHERE cpid = '{user}' AND host = '{host}' ;");
    let rows = sqlx::query(&sql).fetch_all(pool).await?;
    let mut media_vec: Vec<SqliteMedia> = Vec::new();
    for row in rows {
        let name: String = row.get("name");
        let cpid: String = row.get("cpid");
        let host: String = row.get("host");
        let path: String = row.get("path");
        let type_: String = row.get("type");
        let checksum: String = row.get("checksum");
        let size: i64 = row.get("size");
        let date: i64 = row.get("date");
        let local_struct = SqliteMedia {
            name,
            cpid,
            host,
            path,
            type_,
            checksum,
            size,
            date,
        };
        media_vec.push(local_struct);

    }
    Ok(media_vec)
    
}

pub async fn fetch_all_media(
    user: &String,
    pool: &SqlitePool
) -> Result<Vec<SqliteMedia>> {
    let sql = format!("SELECT * FROM media WHERE cpid = '{user}';");
    let rows = sqlx::query(&sql).fetch_all(pool).await?;
    let mut media_vec: Vec<SqliteMedia> = Vec::new();
    for row in rows {
        let name: String = row.get("name");
        let cpid: String = row.get("cpid");
        let host: String = row.get("host");
        let path: String = row.get("path");
        let type_: String = row.get("type");
        let checksum: String = row.get("checksum");
        let size: i64 = row.get("size");
        let date: i64 = row.get("date");
        let local_struct = SqliteMedia {
            name,
            cpid,
            host,
            path,
            type_,
            checksum,
            size,
            date,
        };
        media_vec.push(local_struct);

    }
    Ok(media_vec)
    
}


impl SqliteMedia {
    pub async fn add_media(
        s: Self,
        pool: &SqlitePool
    ) -> Result<()> {
        let sql = format!(
"INSERT INTO media(name, cpid, host, path, type, checksum, size, date)
VALUES ('{}', '{}', '{}', '{}', '{}', '{}', {}, {});",
            s.name,
            s.cpid,
            s.host,
            s.path,
            s.type_,
            s.checksum,
            s.size,
            s.date,
        );
        sqlx::query(&sql).execute(pool).await?;
        Ok(())

    }


}
