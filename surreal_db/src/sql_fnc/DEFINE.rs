
// use//surrealdb::sql;
use crate::db::DB;
// use crate::user::sign_up::User;

pub async fn define_scope_admin() {
    let db = DB.clone();
    let sql = format!("
    DEFINE SCOPE IF NOT EXISTS  admin SESSION 24h
	  SIGNUP ( CREATE admin SET name = $name, pass = crypto::argon2::generate($pass) )
	  SIGNIN ( SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(pass, $pass) )"
);
    let _see = db.query(sql).await.expect("TODO : error msg");
}