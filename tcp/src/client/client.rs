use std::{io::{ErrorKind, Result}, net::IpAddr, os::unix::fs::MetadataExt, path::PathBuf, str::FromStr};

use lib_db::{
    media::checksum, server::host::get_host_ip, types::PgPool, user::{user_jwts::get_jwt, user_struct::{self, User}}
};

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
    let ip = get_host_ip(host.clone(), pool).await.unwrap();
    if 0usize < ip.len() {
        let ip = IpAddr::from_str(ip.as_str()).unwrap();
        let jwt = get_jwt(host.clone(), cred.cpid.clone(), pool).await.unwrap();
        if 0usize < jwt.len() {
            let jwt = Some(jwt);
            let conn = Connection {
                host,
                ip,
                jwt,
                cred
            };
            return Ok(conn)

        }
        else {
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
    let mut state: u8;
    
    let _cred = Cred{
        cpid: usr.cpid,
        name: usr.name,
        paswd: usr.password,
    };

    let conn = check_host(host.clone(), &pool, _cred.clone()).await.unwrap();
    state = connect_tcp(&pool, conn, request.clone()).await.unwrap();
    if state == 8 {
        let conn = check_host(host, &pool, _cred).await.unwrap();
        state = connect_tcp(&pool, conn, request).await.unwrap();
    }
    Ok(state)

} 
