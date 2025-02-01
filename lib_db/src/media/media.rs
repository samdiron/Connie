use sqlx::{PgPool,Row , Result};

use crate::media::checksum;

use super::checksum::get_size;



pub struct Media {
    pub name: String,
    pub cpid: String,
    pub path: String,
    pub checksum: String,
    pub host: String,
    pub type_: String,
    pub size: i64

}



impl Media {
    pub async fn add(self, pool: &PgPool) -> Result<u8> {
        let sql = r#"
            INSERT INTO media(name, cpid, path, checksum, host, type_, size)
            VALUES ($1, $2, $3, $4, $5, $6, $7);
        "#;
        let sum = checksum::get(self.path.as_str()).await?;
        let size = get_size(self.path.as_str()).await?;
        let _res = sqlx::query(sql)
            .bind(self.name)
            .bind(self.cpid)
            .bind(self.path)
            .bind(sum)
            .bind(self.host)
            .bind(self.type_)
            .bind(size)
            .execute(pool)
            .await?
        ;
        Ok(0)
    }

    pub async fn get(
        host: String,
        cpid: String,
        sum: String,
        pool: &PgPool
    ) -> Result<Media> {
        let sql = r#"
            SELECT (name, path, type_ , size) 
            FROM media WHERE host = $1 AND cpid = $2 AND checksum = $3;
        "#;

        let _res = sqlx::query(sql)
            .bind(host.clone())
            .bind(cpid.clone())
            .bind(sum.clone())
            .fetch_one(pool).await?
            
        ;
        let name: String = _res.get("name");
        let size: i64 = _res.get("size");
        let path: String = _res.get("path");
        let type_: String = _res.get("type");
        
        let media = Media {
            name,
            cpid,
            host,
            path,
            size,
            type_,
            checksum: sum
        };
        Ok(media)
    }
}
