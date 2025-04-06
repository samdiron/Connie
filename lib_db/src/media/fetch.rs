use common_lib::bincode;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Result, Row};

use crate::escape_user_input;


/// the s stand for short 
#[derive(Serialize, Deserialize, PartialEq)]
pub struct Smedia {
    pub name: String,
    pub type_: String,
    pub checksum: String,
    pub size: i64,
}

pub async fn get_user_files(
    cpid: String,
    host: String,
    pool: &PgPool
) -> Result<Vec<Smedia>> {
    let sql = format!(
        "SELECT * FROM media WHERE cpid = '{}' AND in_host = '{}' ;",
        escape_user_input(&cpid),
        host
    );
    let _res = sqlx::query(&sql).fetch_all(pool).await?;
    drop(sql);
    let mut media_v: Vec<Smedia> = Vec::new();
    for row in _res {
        let name = row.get("name");
        let type_ = row.get("type");
        let checksum = row.get("checksum");
        let size = row.get("size");

        let media = Smedia { name, type_, checksum, size };
        media_v.push(media);
    } 
    Ok(media_v)
}
impl Smedia {
    pub fn drop(s: Self) {
        drop(s);
    }

    pub fn dz(buf: Vec<u8>) -> Result<Self, bincode::Error> {
        let bind = bincode::deserialize(&buf)?;
        Ok(bind)
    }
    pub fn sz(s: Self) -> Result<Vec<u8>, bincode::Error> {
        let buf = bincode::serialize(&s)?;
        Ok(buf)
    }
}
