use std::{io::{ErrorKind, Result}, net::IpAddr, str::FromStr};

use lib_db::{server::host::get_host_ip, types::PgPool, user::user_jwts::get_jwt};

use super::connector::connect_tcp;
#[allow(dead_code)]
struct Connection {
    host: String,
    ip: IpAddr,
    jwt: Option<String>,
    cred: Option<Cred>
}
#[allow(dead_code)]
pub(crate) struct Cred {
    cpid: String,
    paswd: String,
}

async fn check_host(host: String, pool: &PgPool, cred: Cred ) -> Result<Connection> {
    let ip = get_host_ip(host.clone(), pool).await.unwrap();
    if 0usize < ip.len() {
        let ip = IpAddr::from_str(ip.as_str()).unwrap();
        let jwt = get_jwt(host.clone(), pool).await.unwrap();
        if 0usize < jwt.len() {
            let jwt = Some(jwt);
            let conn = Connection {
                host,
                ip,
                jwt,
                cred: None,
            };
            return Ok(conn)

        }
        else {
            let conn = Connection {
                host,
                ip,
                jwt: None,
                cred: Some(cred)
            };
            return Ok(conn);
        }
    }
    let e = ErrorKind::NotFound;
    Err(e.into()) 
    
    
}

pub async fn client_process(
    host: String,
    _ip: Option<IpAddr>,
    pool: PgPool,
    cpid: String,
    paswd: String,
) -> Result<u8> {
    let _cred = Cred{
        cpid,
        paswd
    };
    if _ip.is_some() {
        connect_tcp(_ip.unwrap(), &pool, Some(_cred), None).await.unwrap();
    } else {
        let conn = check_host(host, &pool, _cred).await.unwrap();
        connect_tcp(conn.ip, &pool, conn.cred, None).await.unwrap();
    };

    



    // let jwt = get_jwt(host.clone(), &pool).await.unwrap();
    // if let Some(inner_ip) = ip {
    //     let _ok = connect_tcp(inner_ip, jwt.clone(), &pool).await.is_ok();
    // }
    // else {
    //     let _ip = get_host_ip(host, &pool).await.unwrap();
    //     let ip = IpAddr::from_str(_ip.as_str()).unwrap();
    //     let _ok = connect_tcp(ip, jwt, &pool).await?;
    // }
    // 
    Ok(1)

} 
