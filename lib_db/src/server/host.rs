use common_lib::log::debug;
use sqlx::Row;
use sqlx::query;
use sqlx::Result;
use sqlx::PgPool;

use crate::media::fetch::Smedia;
use crate::sha256::digest;
use crate::escape_user_input;
use crate::media::media::Media;
use super::server_struct::Server;
use crate::sqlite::sqlite_host::SqliteHost;

/// gets the host info used for bind --server 
pub async fn get_host_info(
    name: &String,
    password: &String,
    pool: &PgPool,
    is_hashed: bool,
) -> Result<Server> {
    let password = if !is_hashed {
        digest(password)
    }else {password.to_owned()};

    let sql = format!(
        " 
        SELECT * 
        FROM server
        WHERE name = '{}' AND password = '{}';",
        escape_user_input(name),
        escape_user_input(&password)
    );

    let row = sqlx::query(&sql)
    .fetch_one(pool)
    .await?;
    
    let s = Server{
        cpid: row.get("cpid"),
        name: row.get("name"),
        host: row.get("host"),
        pri_ip: row.get("pri_ip"),
        pub_ip: row.get("pub_ip"),
        memory: row.get("memory"),
        max_conn: row.get("max_conn"),
        password: row.get("password")

    };
    Ok(s)

}



/// return Smedia for fetch requests
pub async fn fetch_host_public_files(
    server: &SqliteHost,
    pool: &PgPool
) -> Result<Vec<Smedia>> {
    let sql = format!("
SELECT (name, size, path, type, checksum) 
FROM media 
WHERE in_host = cpid AND in_host = '{}' ;
", escape_user_input(&server.cpid));
    let vec_res = query(&sql).fetch_all(pool).await?;
    let mut media_vec: Vec<Smedia> = Vec::with_capacity(vec_res.len()); 
    for res in vec_res {
        let name = res.get("name");
        let size = res.get("size");
        let type_ = res.get("type");
        let checksum = res.get("checksum");
        let m: Smedia = Smedia {
            name,
            size,
            type_,
            checksum,
        };
        media_vec.push(m);
    };
    Ok(media_vec)
}   


// returns Media for checking process
pub async fn get_host_public_files(
    server: &SqliteHost,
pool: &PgPool
) -> Result<Vec<Media>> {
let sql = format!("
SELECT *
FROM media 
WHERE in_host = cpid AND in_host = '{}' ;
", escape_user_input(&server.cpid));
    let vec_res = query(&sql).fetch_all(pool).await?;
    let mut media_vec: Vec<Media> = Vec::with_capacity(vec_res.len()); 
    debug!("PUBLIC_FILES: {} records found", vec_res.len());
    for res in vec_res {
        let name: String = res.get("name");
        let size: i64 = res.get("size");
        let path: String = res.get("path");
        let type_: String = res.get("type");
        let checksum: String = res.get("checksum");
        let cpid: String = server.cpid.clone();
        let in_host: String = server.cpid.clone();
        let m: Media = Media {
            name,
            cpid,
            size,
            path,
            type_,
            in_host,
            checksum,
        };
        media_vec.push(m);
    };
    Ok(media_vec)
}   

pub async fn update_server_pri_ip(
    server_cpid: &String,
    ip: &String,
    pool: &PgPool
) -> Result<u8> {
    let sql = format!("
UPDATE server
SET pri_ip = '{}'
WHERE cpid = '{}' ;
",  escape_user_input(ip),
    escape_user_input(server_cpid)
    );
    debug!("updating server public ip");
    let res = query(&sql).execute(pool).await?;
    drop(sql);
    Ok(res.rows_affected() as u8 )
}


pub async fn update_server_pub_ip(
    server_cpid: &String,
    ip: &String,
    pool: &PgPool
) -> Result<u8> {
    let sql = format!("
UPDATE server
SET pub_ip = '{}'
WHERE cpid = '{}' ;
",  escape_user_input(ip),
    escape_user_input(server_cpid)
    );
    debug!("updating server private ip");
    let res = query(&sql).execute(pool).await?;
    Ok(res.rows_affected() as u8 )
}




pub async fn get_host_ip(host: String, pool: &PgPool) -> Result<String> {
    let sql = r#"
        SELECT ip FROM server WHERE host = $1;
    "#;
    let res = sqlx::query(sql).bind(host).fetch_one(pool).await?;
    let ip = res.get("ip");
    Ok(ip)
}
