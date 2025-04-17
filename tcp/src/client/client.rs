use std::{
    io::Result,
    net::IpAddr
};

use lib_db::types::SqlitePool;
use lib_db::sqlite::{
        sqlite_jwt::get_jwt,
        sqlite_host::SqliteHost,
        sqlite_user::SqliteUser,
    }
;

use tokio_rustls::rustls::ClientConnection;

use crate::common::request::RQM;
use crate::client::connector::connect_tcp;


#[allow(dead_code)]
pub(crate) struct Connection {
    pub jwt: Option<String>,
    pub user: SqliteUser,
    pub server: SqliteHost,
    pub ip: Option<IpAddr>,
    pub port: Option<u16>,
}

pub use crate::client::connector::signup_process;






pub struct Client {
    pub inner: ClientConnection
}






/// spins up a client process that could be use inside a task
pub async fn client_process(
    pool: SqlitePool,
    usr: SqliteUser,
    server: SqliteHost,
    port: Option<u16>,
    ip: Option<IpAddr>,
    check_for_sum: Option<bool>,
    request: Option<RQM>,
    log_in: Option<bool>,

) -> Result<u8> {
    let state: u8;
    
    let jwt = get_jwt(&server.cpid, &usr.cpid, &pool).await.unwrap();
    let conn = if log_in.is_some() && log_in.unwrap(){
        Connection {
            jwt: None,
            user: usr,
            ip,
            port,
            server,
        }

    }
    else {
        assert!(jwt.is_some());
        let conn = Connection {
            jwt,
            user: usr,
            ip,
            port,
            server
        };
        conn
    };  
    let request = if request.is_some() {
        request
    } else {
        drop(request);
        None
    };
    state = connect_tcp(&pool, conn, check_for_sum, request).await.unwrap();
    Ok(state)

} 
