use common_lib::log::debug;
use serde::{Deserialize, Serialize};
use sqlx::{Result, Row, SqlitePool};

use crate::{escape_user_input, media::fetch::Smedia};



#[derive(Serialize, Deserialize, Clone, PartialEq)]
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

pub(in crate::sqlite) async fn create_table(pool: &SqlitePool) 
-> Result<()>{
    debug!("SQLITE: {SQL}");
    sqlx::query(SQL).execute(pool).await?;
    Ok(())
}


pub async fn fetch_all_media_from_host_number(
    host: &String,
    user: &String,
    pool: &SqlitePool
) -> u64 {
    let user = escape_user_input(&user);
    let host = escape_user_input(&host);
    let sql = format!(r#"
SELECT count(*)
FROM media 
WHERE cpid = '{user}' AND host = '{host}' ;
    "#);
    let res = sqlx::query(&sql).fetch_one(pool).await.unwrap();
    drop(sql);
    let count: i64 =res.get(0usize);
    return count as u64
}


pub async fn fetch_all_media_from_host_smedia(
    host: &String,
    user: &String,
    pool: &SqlitePool
) -> Result<Vec<Smedia>> {
    let user = escape_user_input(&user);
    let host = escape_user_input(&host);
    let sql = format!(r#"
SELECT *
FROM media
WHERE cpid = '{user}' AND host = '{host}' ;
    "#);
    let rows = sqlx::query(&sql).fetch_all(pool).await?;
    drop(sql);
    let mut media_vec: Vec<Smedia> = Vec::new();
    for row in rows {
        let name: String = row.get("name");
        let type_: String = row.get("type");
        let checksum: String = row.get("checksum");
        let size: i64 = row.get("size");
        let local_struct = Smedia {
            name,
            type_,
            checksum,
            size,
        };
        media_vec.push(local_struct);

    }
    Ok(media_vec)
    
}


   
pub async fn fetch_all_public_files_from_host(
    host: &String,
    pool: &SqlitePool
) -> Result<Vec<Smedia>> {
    let host = escape_user_input(&host);
    let sql = format!(r#"
SELECT *
FROM media
WHERE cpid = '{}' AND host = '{}' ;
    "#, host, host);
    let rows = sqlx::query(&sql).fetch_all(pool).await?;
    drop(sql);
    let mut media_vec: Vec<Smedia> = Vec::new();
    for row in rows {
        let name: String = row.get("name");
        let type_: String = row.get("type");
        let checksum: String = row.get("checksum");
        let size: i64 = row.get("size");
        let local_struct = Smedia {
            name,
            type_,
            checksum,
            size,
        };
        media_vec.push(local_struct);

    }
    Ok(media_vec)
    
}

pub async fn fetch_all_media_from_host(
    host: &String,
    user: &String,
    pool: &SqlitePool
) -> Result<Vec<SqliteMedia>> {
    let user = escape_user_input(&user);
    let host = escape_user_input(&host);
    let sql = format!(r#"
SELECT * 
FROM media 
WHERE cpid = '{user}' AND host = '{host}';
    "#);
    let rows = sqlx::query(&sql).fetch_all(pool).await?;
    drop(sql);
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
    let cpid = escape_user_input(user);
    let sql = format!("
SELECT * 
FROM media 
WHERE cpid = '{cpid}';
    ");
    let rows = sqlx::query(&sql).fetch_all(pool).await?;
    drop(sql);
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
pub async fn sqlite_delete_media(
    host: &String,
    cpid: &String,
    checksum: &String,
    pool: &SqlitePool
) {
    let host = escape_user_input(host);
    let cpid = escape_user_input(cpid);
    let checksum = escape_user_input(checksum);
    let sql = format!(r#"
DELETE FROM media
WHERE cpid = '{}' AND checksum = '{}' AND host = '{}';
    "#,
        cpid,
        checksum,
        host
    );
    let _res = sqlx::query(&sql).execute(pool).await.unwrap();
    drop(sql);
}

pub async fn sqlite_media_exists(
    host: &String,
    cpid: &String,
    checksum: &String,
    pool: &SqlitePool
) -> bool {
    let sql = format!(r#"
    SELECT
    count(*) FROM media 
    WHERE checksum = '{}' AND cpid = '{}' AND host = '{}';
    "#,
        escape_user_input(checksum),
        escape_user_input(cpid),
        escape_user_input(host),
    );
    let res = sqlx::query(&sql).fetch_one(pool).await.unwrap();
    drop(sql);
    let count: i64 = res.get(0usize);
    return count == 1;

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
        drop(sql);
        Ok(())

    }
    pub async fn exists(s: &Self, pool: &SqlitePool) -> bool {
        let sql = format!(r#"
        SELECT count(*)
        FROM media 
        WHERE checksum = '{}' AND cpid = '{}' AND host = '{}' ;
        "#,
        escape_user_input(&s.checksum),
        escape_user_input(&s.cpid),
        escape_user_input(&s.host),
    );
        let res = sqlx::query(&sql).fetch_one(pool).await.unwrap();
        drop(sql);
        let count: i64 = res.get(0usize);
        return count == 1;

    } 
    
}
