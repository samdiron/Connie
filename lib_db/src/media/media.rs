
use sqlx::{PgPool,Row , Result};
use super::checksum::get_size;



pub struct Media {
    pub name: String,
    pub cpid: String,
    pub path: String,
    pub checksum: String,
    pub in_host: String,
    pub type_: String,
    pub size: i64

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
WHERE name: '{name}' 
AND size = {size} 
AND host = '{host}' 
AND type = '{type_}'
AND cpid = '{cpid}';");
    let _res = sqlx::query(&sql).fetch_one(pool).await.unwrap();
    let count: i8 = _res.get("count");
    return if count == 1 {
        true
    } else {false}

}



impl Media {

    pub async fn post(self, pool: &PgPool) -> Result<u8> {
        let sql = r#"
            INSERT INTO media(name, cpid, path, checksum, in_host, type, size)
            VALUES ($1, $2, $3, $4, $5, $6, $7);
        "#;
        let size = get_size(self.path.as_str()).await?;
        let _res = sqlx::query(sql)
            .bind(self.name)
            .bind(self.cpid)
            .bind(self.path)
            .bind(self.checksum)
            .bind(self.in_host)
            .bind(self.type_)
            .bind(size)
            .execute(pool)
            .await?
        ;
        Ok(0)
    }
    
    pub async fn get(
        host_cpid: &String,
        cpid: &String,
        sum: &String,
        pool: &PgPool
    ) -> Result<Media> {
        let sql = r#"
            SELECT * 
            FROM media WHERE in_host = $1 AND cpid = $2 AND checksum = $3;
        "#;

        let _res = sqlx::query(sql)
            .bind(host_cpid)
            .bind(cpid)
            .bind(sum)
            .fetch_one(pool).await?
            
        ;
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
