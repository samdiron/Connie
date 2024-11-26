use crate::db::DB;
use surrealdb::Response;

pub static USER_SESSION_TIME: u8 = 12;
pub static ADMIN_SEEION_TIME: u8 = 24;

pub async fn define_scope_admin() -> surrealdb::Result<Response> {
    let db = DB.clone();
    let sql = format!("
    DEFINE SCOPE IF NOT EXISTS  admin SESSION {ADMIN_SEEION_TIME}h
	  SIGNUP ( CREATE admin SET name = $name, user_name = $user_name, pass = crypto::argon2::generate($pass), is_admin = $is_admin  )
	  SIGNIN ( SELECT * FROM admin WHERE cpid = $cpid AND crypto::argon2::compare(pass, $pass) )");
    let query_result = db.query(sql).await;
    let err_is_ok = query_result.is_ok();
    if err_is_ok {
        let query_result = query_result.unwrap();
        println!("query executed successfully");
        Ok(query_result)
    } else {
        let err = query_result.unwrap_err();
        Err(err)
    }
}

pub async fn define_scope_user() -> surrealdb::Result<Response> {
    let db = DB.clone();
    let sql = format!("
    DEFINE SCOPE IF NOT EXISTS  user SESSION {USER_SESSION_TIME}h
	  SIGNUP ( CREATE admin SET name = $name, user_name = $user_name, pass = crypto::argon2::generate($pass), is_admin = $is_admib  )
	  SIGNIN ( SELECT * FROM admin WHERE cpid = $cpid AND crypto::argon2::compare(pass, $pass) )");
    let query_result = db.query(sql).await;
    let err_is_ok = query_result.is_ok();
    if err_is_ok {
        let query_result = query_result.unwrap();
        println!("query executed successfully");
        Ok(query_result)
    } else {
        let err = query_result.unwrap_err();
        Err(err)
    }
}

// TODO: user scope function

// pub async fn define_scope_user() {
//     let db = DB.clone();
//     let sql = ""
// }
