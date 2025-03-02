use std::{
    io::{ErrorKind, Result}, net::IpAddr, path::PathBuf, str::FromStr
};

use lib_db::{
    server::host::get_host_ip, sqlite::get_sqlite_conn, types::PgPool, user::{
        user_jwts::get_jwt,
        user_struct::User
    }
};
use common_lib::{log::debug, path::SQLITEDB_PATH};

use crate::common::request::RQM;

use super::connector::connect_tcp;

#[allow(dead_code)]
pub(crate) struct Connection {
    pub host: String,
    pub ip: IpAddr,
    pub jwt: Option<String>,
    pub cred: Cred,
}

#[derive(Clone)]
pub(crate) struct Cred {
    pub cpid: String,
    pub name: String,
    pub paswd: String,
}

/// it tries to get a jwt and if the jwt is not valid it will instead load the user cred 
async fn check_host(host: String, pool: &PgPool, cred: Cred ) -> Result<Connection> {
    let _spool = get_sqlite_conn(&SQLITEDB_PATH.to_owned()).await.unwrap();

    let ip = get_host_ip(host.clone(), pool).await.unwrap();
    if 0usize < ip.len() {
        let ip = IpAddr::from_str(ip.as_str()).unwrap();
        let  res = get_jwt(host.clone(), cred.cpid.clone(), pool).await;
        if  res.is_ok() {
            let res = res.unwrap();
            debug!("CLIENT: found jwt for Host: {}",&res);
            let jwt = Some(res);
            let conn = Connection {
                host,
                ip,
                jwt,
                cred
            };
            return Ok(conn)

        }
        else {
            let e = res.unwrap_err();
            eprintln!("error while trying to get jwt: {:#?}", e);
            let conn = Connection {
                host,
                ip,
                jwt: None,
                cred
            };
            return Ok(conn);
        }
    }
    let e = ErrorKind::NotFound;
    Err(e.into()) 
    
    
}



/// spins up a client process that could be use inside a task
pub async fn client_process(
    host: String,
    _ip: Option<IpAddr>,
    pool: PgPool,
    usr: User,
    request: RQM,

) -> Result<u8> {
    let state: u8;
    
    let _cred = Cred{
        cpid: usr.cpid,
        name: usr.name,
        paswd: usr.password,
    };

    let conn = check_host(host.clone(), &pool, _cred.clone()).await.unwrap();
    state = connect_tcp(&pool, conn, request.clone()).await.unwrap();
    println!("logged in now you can retry to make the request");
    Ok(state)

} 
