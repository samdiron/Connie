use common_lib::cheat_sheet::gethostname;
use sqlx::{PgPool, Result, Row};


/// the s stand for short 
pub struct Smedia {
    pub name: String,
    pub type_: String,
    pub checksum: String,
    pub size: i64
}

pub async fn get_user_files(
    cpid: String,
    pool: &PgPool
) -> Result<Vec<Smedia>> {
    let host = gethostname();
    let sql = format!("SELECT (name, type, checksum, size) FROM media WHERE cpid = {} AND in_host = {:?} ;",&cpid ,host);
    let _res = sqlx::query(&sql).fetch_all(pool).await?;
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
