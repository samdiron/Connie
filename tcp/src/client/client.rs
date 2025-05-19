use std::{
    io::Result,
    net::IpAddr
};

use lib_db::sqlite::sqlite_jwt::delete_user_jwt;
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

use super::connector::login_request;






pub struct Client {
    pub inner: ClientConnection
}







pub async fn client_login_process(
    pool: &SqlitePool,
    usr: SqliteUser,
    server: SqliteHost,
    port: Option<u16>,
    ip: Option<IpAddr>,
    passwd: String,
) -> Result<u8> {
    let conn = Connection {
        jwt: None,
        user: usr,
        ip,
        port,
        server
    };
    delete_user_jwt(
        pool,
        &conn.user.cpid,
        &conn.server.cpid
    ).await;
    let status = login_request(pool, conn, passwd).await?;
    Ok(status)
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
    no_tls: bool,
) -> Result<u8> {

    let jwt = get_jwt(
        &server.cpid,
        &usr.cpid,
        &pool
    ).await.unwrap();
    assert!(jwt.is_some());

    let conn = Connection {
        jwt,
        user: usr,
        ip,
        port,
        server
    };

    let state = connect_tcp(
        &pool,
        conn,
        check_for_sum,
        request,
        no_tls,
    ).await.unwrap();
    Ok(state)

} 
