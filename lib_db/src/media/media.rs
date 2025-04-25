
use sqlx::{PgPool,Row , Result};
use super::checksum::get_size;
use crate::escape_user_input;


pub struct Media {
    pub name: String,
    pub cpid: String,
    pub path: String,
    pub checksum: String,
    pub in_host: String,
    pub type_: String,
    pub size: i64

}


pub async fn check_if_media_exist_wchecksum(
    media: &Media,
    pool: &PgPool
) -> bool {
    let sql = format!("
SELECT count(1) FROM media
WHERE size = {} 
AND in_host = '{}' 
AND cpid = '{}'
AND checksum = '{}' ;",
        media.size,
        escape_user_input(&media.in_host),
        escape_user_input(&media.cpid),
        escape_user_input(&media.checksum),
);
     let _count = sqlx::query(&sql).fetch_one(pool).await;
    drop(sql);
    if _count.is_err() {return false}else {
    let count: i64 = _count.unwrap().get("count");
    if count == 1 {
        return true
    } else {
        return false
    }
    }

}


pub async fn check_if_media_exist(
    cpid: &String,
    host: &String,
    name: &String,
    type_: &String,
    size: i64,
    pool: &PgPool
) -> bool {
    let sql = format!("
SELECT count(1) FROM media
WHERE name = '{}' 
AND size = {} 
AND in_host = '{}' 
AND type = '{}'
AND cpid = '{}';",
        escape_user_input(name),
        size,
        escape_user_input(host),
        escape_user_input(type_),
        escape_user_input(cpid),
);
     let _count = sqlx::query(&sql).fetch_one(pool).await;
    drop(sql);
    if _count.is_err() {return false}else {
    let count: i64 = _count.unwrap().get("count");
    if count == 1 {
        return true
    } else {
        return false
    }
    }

}

pub async fn delete_media(s: Media, pool: &PgPool) -> Result<u64> {
    let sql = format!(r#"
DELETE
FROM media 
WHERE checksum = '{}' AND cpid = '{}' AND in_host = '{}'; 
    "#,
        escape_user_input(&s.checksum),
        escape_user_input(&s.cpid),
        escape_user_input(&s.in_host),
    );
    let res = sqlx::query(&sql).execute(pool).await?;
    drop(sql);
    let rows = res.rows_affected();
    Ok(rows)
}


impl Media {

    pub async fn post(self, pool: &PgPool) -> Result<u8> {
        let size = get_size(self.path.as_str()).await?;
        let sql = format!("
INSERT INTO media(
    name,
    cpid,
    path,
    checksum,
    in_host,
    type,
    size
) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', {});
        ",
        escape_user_input(&self.name),
        escape_user_input(&self.cpid),
        escape_user_input(&self.path),
        self.checksum,
        escape_user_input(&self.in_host),
        escape_user_input(&self.type_),
        size,
        );
        let _res = sqlx::query(&sql)
            .execute(pool)
            .await?
        ;
        drop(sql);
        drop(self);
        Ok(0)
    }
    
    pub async fn get(
        host_cpid: &String,
        cpid: &String,
        sum: &String,
        pool: &PgPool
    ) -> Result<Media> {
        let sql = format!("
SELECT * 
FROM media
WHERE in_host = '{}' AND cpid = '{}' AND checksum = '{}';
        ",
            escape_user_input(host_cpid),
            escape_user_input(cpid),
            escape_user_input(sum)
        );

        let _res = sqlx::query(&sql)
            .bind(host_cpid)
            .bind(cpid)
            .bind(sum)
            .fetch_one(pool).await?
            
        ;
        drop(sql);
        let name: String = _res.get("name");
        let size: i64 = _res.get("size");
        let path: String = _res.get("path");
        let type_: String = _res.get("type");
        let cpid: String = _res.get("cpid");
        let in_host: String = _res.get("in_host");
        let checksum: String = _res.get("checksum");
        
        let media = Media {
            name,
            cpid,
            in_host,
            path,
            size,
            type_,
            checksum
        };
        Ok(media)
    }

}
