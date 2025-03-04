use std::{
    io::{ErrorKind, Result}, net::IpAddr
};

use lib_db::{
    sqlite::{
        sqlite_host::{get_host_ip, SqliteHost}, sqlite_jwt::get_jwt, sqlite_user::SqliteUser
    },
    types::SqlitePool
};
use common_lib::log::{debug, warn};

use crate::common::request::RQM;

use super::connector::connect_tcp;
#[allow(dead_code)]
pub(crate) struct Connection {
    pub jwt: Option<String>,
    pub user: SqliteUser,
    pub server: SqliteHost,
}

// #[derive(Clone)]
// pub(crate) struct Cred {
//     pub cpid: String,
//     pub name: String,
//     pub paswd: String,
// }

/// it tries to get a jwt and if the jwt is not valid it will instead load the user cred 
// async fn check_host(host_name: String, host: String, user_name: String, pool: &SqlitePool, cred: Cred) -> Result<Connection> {
//     let user = 
//     
//     let ip = get_host_ip(&name &host, pool).await;
//     if ip.is_ok() {
//         let ip = ip.unwrap();
//         let pri_ip = ip.0;
//         let pub_ip = ip.1;
//         let  res = get_jwt(&host, &cred.cpid, pool).await;
//         if  res.is_ok() {
//             let res = res.unwrap();
//             debug!("CLIENT: found jwt for Host: {}",&res);
//             let jwt = Some(res);
//             let conn = Connection {
//                 host,
//                 pub_ip,
//                 pri_ip,
//                 jwt,
//                 cred
//             };
//             return Ok(conn)
//
//         }
//         else {
//             let e = res.unwrap_err();
//             warn!("error while trying to get jwt: {:#?}", e);
//             let conn = Connection {
//                 host,
//                 pri_ip,
//                 pub_ip,
//                 jwt: None,
//                 cred
//             };
//             return Ok(conn);
//         }
//     }
//     let e = ErrorKind::NotFound;
//     Err(e.into()) 
//     
//     
// }
//

pub use crate::client::connector::signup_process;

/// spins up a client process that could be use inside a task
pub async fn client_process(
    pool: SqlitePool,
    usr: SqliteUser,
    server: SqliteHost,
    request: RQM,

) -> Result<u8> {
    let state: u8;
    
    let jwt = get_jwt(&server.cpid, &usr.cpid, &pool).await;
    let conn = if jwt.is_ok() {
        let jwt = Some(jwt.unwrap());
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
    state = connect_tcp(&pool, conn, request).await.unwrap();
    Ok(state)

} 
