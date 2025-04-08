use common_lib::log::debug;
use common_lib::log::error;
use common_lib::log::warn;
use jsonwebtoken::get_current_timestamp;
use sqlx::Result;
use sqlx::SqlitePool;
use sqlx::Row;

use crate::jwt::exp_gen;


const SQL: &str = "
CREATE TABLE jwt(host TEXT,
    cpid TEXT,
    exp BIGINT,
    token TEXT
);";


pub(in crate::sqlite) async fn create_table(
    pool: &SqlitePool
) -> Result<()>{
    debug!("SQLITE: {SQL}");
    sqlx::query(SQL).execute(pool).await.unwrap();
    Ok(())
}


pub async fn get_jwt(
    host: &String,
    cpid: &String,
    pool: &SqlitePool
) -> Result<Option<String>> {
    let now = get_current_timestamp();
    let sql = format!(r#"
    SELECT token
    FROM jwt 
    WHERE host = '{host}' AND exp > {now} AND cpid = '{cpid}' ;"#);
    let _res = sqlx::query(&sql).fetch_one(pool).await;
    drop(sql);
    if _res.is_ok() {
        let _res= _res.unwrap();
        let token: String = _res.get("token");
        return Ok(Some(token))
    } else {
        return Ok(None)
    };
} 
pub async fn delete_jwt(token: &String, pool: &SqlitePool) -> Result<()>{
    let sql = format!("DELETE FROM jwt WHERE token = '{token}' ;");
    let res = sqlx::query(&sql).execute(pool).await?;
    drop(sql);
    debug!("rows affected: {}", res.rows_affected());
    Ok(())
}
pub async fn add_jwt(
    host_cpid: &String,
    token: &String,
    user_cpid: &String,
    pool: &SqlitePool
) {
    let exp = exp_gen();
    let sql = format!(r#"
    INSERT INTO jwt(host, cpid, exp, token) 
    VALUES('{host_cpid}', '{user_cpid}', {exp},'{token}');
    "#);
    let res = sqlx::query(&sql).execute(pool).await;
    drop(sql);
    if res.is_ok() {
        debug!("jwt added");
    }else {
        error!("could not add jwt");
    }
}

pub async fn delete_user_jwt(pool: &SqlitePool, cpid: &String) {
    let sql = format!("DELETE FROM jwt WHERE cpid = '{}' ;", cpid);
    let res = sqlx::query(&sql).execute(pool).await;
    drop(sql); 
    if res.is_ok(){
        debug!("DELETED {} jwts from db", res.unwrap().rows_affected())
    } else {
        warn!("fn delete_expd: {:#?}", res.unwrap_err())
    }


}

pub async fn delete_expd_jwt(pool: &SqlitePool) {
    let now = get_current_timestamp();
    let sql = format!("DELETE FROM jwt WHERE exp < {now} ;");
    let res = sqlx::query(&sql).execute(pool).await;
    drop(sql);
    if res.is_ok(){
        debug!("DELETED {} jwts from db", res.unwrap().rows_affected())
    } else {
        warn!("fn delete_expd: {:#?}", res.unwrap_err())
    }


}
