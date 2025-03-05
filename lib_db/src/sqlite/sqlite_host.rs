use std::net::IpAddr;
use std::str::FromStr;

use common_lib::bincode;
use common_lib::log::debug;
use serde::Deserialize;
use serde::Serialize;
use sqlx::Result;
use sqlx::SqlitePool;
use sqlx::Row;

const SQL: &str  = r#"
CREATE TABLE host(
    name TEXT,
    pub_ip TEXT,
    pri_ip TEXT,
    cpid TEXT,
    host TEXT,
    port INT
);"#;

pub(in crate::sqlite) async fn create_table(pool: &SqlitePool) -> Result<()>{
    debug!("SQLITE: {SQL}");
    sqlx::query(SQL).execute(pool).await?;
    Ok(())
}
#[derive(Serialize, Deserialize, Clone)]
pub struct SqliteHost {
    pub name: String,
    pub cpid: String,
    pub host: String,
    pub port: u16,
    pub pub_ip: String,
    pub pri_ip: String,
}

pub async fn get_host_ip(
    name: &String,
    host: &String,
    pool: &SqlitePool
) -> Result<(IpAddr, IpAddr), sqlx::Error > {
    let sql = format!("SELECT (pri_ip, pub_ip) FROM host WHERE name = '{name}' AND host = '{host}' ;");
    let res = sqlx::query(&sql).fetch_one(pool).await?;
    let pub_ip:String = res.get("pub_ip");
    let pri_ip:String = res.get("pri_ip");
    let pri =  IpAddr::from_str(&pri_ip).unwrap();
    let public = IpAddr::from_str(&pub_ip).unwrap();
    let vector = (pri, public);
    Ok(vector)
}

pub async fn fetch_server(
    name: &String,
    host: &String,
    pool: &SqlitePool
) -> SqliteHost {
    let sql = format!("SELECT * FROM host WHERE name = '{name}' AND host = '{host}'");
    let _res = sqlx::query(&sql).fetch_one(pool).await.unwrap();
    let name: String = _res.get("name");
    let cpid: String = _res.get("cpid");
    let host: String = _res.get("host");
    let pri_ip: String = _res.get("pri_ip");
    let pub_ip: String = _res.get("pub_ip");
    let port: u16 = _res.get("port");
    SqliteHost {
        name,
        cpid,
        host,
        port,
        pub_ip,
        pri_ip
    }
}

impl SqliteHost {
    pub fn dz(v: Vec<u8>) -> Result<Self, bincode::Error> {
        let dzd: Self = bincode::deserialize(&v)?;
        Ok(dzd)
    }

    pub fn sz(s: Self) -> Result<Vec<u8>, bincode::Error> {
        let szd = bincode::serialize(&s)?;
        drop(s);
        Ok(szd)
    }
    pub async fn update_pub_ip(s: &Self, ip: IpAddr, pool: &SqlitePool) -> Result<()> {
        let ip = ip.to_string();
        let sql = format!("
            UPDATE host
            SET pub_ip = '{ip}'
            WHERE cpid = '{}';", &s.cpid
        );
        sqlx::query(&sql).execute(pool).await?;
        Ok(())
    }
    pub async fn update_pri_ip(s: &Self, ip: IpAddr, pool: &SqlitePool) -> Result<()> {
        let ip = ip.to_string();
        let sql = format!("
            UPDATE host
            SET pri_ip = '{ip}'
            WHERE cpid = '{}';", &s.cpid
        );
        sqlx::query(&sql).execute(pool).await?;
        Ok(())
    }
    /// note this function takes into account that host is OsStr aka 'host'
    pub async fn new(s: Self, pool: &SqlitePool) {
        let sql = format!(
            "INSERT INTO host(name, cpid, host, port, pub_ip, pri_ip) VALUES('{}','{}','{}', {}, '{}','{}');",
            s.name,
            s.cpid,
            s.host,
            s.port,
            s.pub_ip,
            s.pri_ip,
        );
        debug!("exec sql: {}", &sql);
        let _res = sqlx::query(&sql).execute(pool).await.unwrap();
        debug!("sqlite lines af: {} ",_res.rows_affected())
    }
    pub async fn delete(s: Self, pool: &SqlitePool) {
        let sql = format!(
            "DELETE FROM host WHERE name = {} AND cpid = {}",
            &s.name,
            &s.cpid,
        );
        let _res = sqlx::query(&sql)
            .execute(pool)
            .await
            .expect("could not delete host")
        ;
        debug!("sqlite db rows affected: {}", _res.rows_affected());
    }

    
}
