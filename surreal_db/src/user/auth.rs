use crate::db::DB;
use surrealdb::opt::auth::Jwt;
pub async fn jwt_auth(jwt: String) -> bool {
    let mut jwt = Jwt::from(jwt);
    let mut db = DB.clone();
    let res = db.authenticate(jwt);
    println!("{:?}", res);
    return true;
}
