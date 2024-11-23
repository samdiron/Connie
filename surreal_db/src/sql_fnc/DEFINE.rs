// use//surrealdb::sql;
use crate::db::DB;
// use crate::user::sign_up::User;

pub async fn define_scope_admin() {
    let db = DB.clone();
    let sql = "
    DEFINE SCOPE IF NOT EXISTS  admin SESSION 24h
	  SIGNUP ( CREATE admin SET name = $name, user_name = $user_name, pass = crypto::argon2::generate($pass), is_admin = $is_admib  )
	  SIGNIN ( SELECT * FROM admin WHERE cpid = $cpid AND crypto::argon2::compare(pass, $pass) )"
;
    let _see = db.query(sql).await.expect("TODO : error msg");
}
// TODO: user scope function

// pub async fn define_scope_user() {
//     let db = DB.clone();
//     let sql = ""
// }
