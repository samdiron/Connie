use std::
    io::Result
;

use lib_db::{
    sqlite::{
        sqlite_host::SqliteHost,
        sqlite_jwt::get_jwt,
        sqlite_user::SqliteUser
    },
    types::SqlitePool
};
// use common_lib::log::{debug, warn};

use crate::common::request::RQM;

use super::connector::connect_tcp;
#[allow(dead_code)]
pub(crate) struct Connection {
    pub jwt: Option<String>,
    pub user: SqliteUser,
    pub server: SqliteHost,
}

pub use crate::client::connector::signup_process;






/// spins up a client process that could be use inside a task
pub async fn client_process(
    pool: SqlitePool,
    usr: SqliteUser,
    server: SqliteHost,
    check_for_sum: Option<bool>,
    request: RQM,

) -> Result<u8> {
    let state: u8;
    
    let jwt = get_jwt(&server.cpid, &usr.cpid, &pool).await.unwrap();
    let conn = if jwt.is_some() {
        let conn = Connection {
            jwt,
            user: usr,
            server
        };
        conn
    } else {
        Connection {
            jwt: None,
            user: usr,
            server,
        }
    };
    state = connect_tcp(&pool, conn, check_for_sum,request).await.unwrap();
    Ok(state)

} 
