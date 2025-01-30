use std::io::{Error, ErrorKind, Result};

use lib_db::{jwt::{self, exp_gen, validate_jwt_claim, Claim}, types::PgPool, user::user_struct::vaildate_claim};
use tokio::{io::AsyncWriteExt, net::TcpStream};

#[allow(dead_code)]
pub const JWT_AUTH: &str = "0";

#[allow(dead_code)]
pub const LOGIN_CRED: &str = "1";

#[allow(dead_code)]
pub const SIGNIN_CRED: &str = "2";

#[allow(dead_code)]
pub const PACKET_SIZE: u16 = 65535;

//request Head

#[allow(dead_code)]
pub const GET: &str = "!G: ";

#[allow(dead_code)]
pub const POST: &str = "!P: ";

#[allow(dead_code)]
pub const DELETE: &str = "!D: ";

#[allow(dead_code)]
pub const REQUEST_HEADER: &str = "!rq: ";

#[allow(dead_code)]
pub const SPLIT: &str = "\n";

#[allow(dead_code)]
pub const END: &str = "\0";

#[allow(dead_code)]
pub const JWT_HEAD: &str = "JWT: ";

#[allow(dead_code)]
pub const INVALID_RQ: &str = "INVALID";

#[allow(dead_code)]
pub const PASSWORD_HEADER: &str = "PassWd: ";

#[allow(dead_code)]
pub const CPID_HEADER: &str = "CpId: ";

#[allow(dead_code)]
pub fn format_jwt_request(jwt: String, request: String) -> String {
    let string = format!("{JWT_AUTH}{SPLIT}{JWT_HEAD}{jwt}{SPLIT}{REQUEST_HEADER}{request}{END}");
    string
}

#[allow(dead_code)]
pub fn format_login_request(cred: Vec<&str>) -> String {
    let cpid = cred[0];
    let password = cred[1];
    let string =
        format!("{LOGIN_CRED}{SPLIT}{CPID_HEADER}{cpid}{SPLIT}{PASSWORD_HEADER}{password}{END}");
    string
}

#[allow(dead_code)]
pub fn split_request(request: String) -> Vec<String> {
    let split_arr: Vec<&str> = request.split(SPLIT).collect();
    let mut local_request: Vec<String> = Vec::new();
    match split_arr[0] {
        JWT_AUTH => {
            //adding the jwt
            local_request.push(JWT_HEAD.to_string());
            let jwt_head: &str = split_arr[1];
            let jwt = jwt_head.to_string().split_off(JWT_HEAD.char_indices().count());
            local_request.push(jwt.to_string());
            //adding the request
            local_request.push(split_arr[2].to_string());
        }
        LOGIN_CRED => {
            local_request.push(LOGIN_CRED.to_string());
            local_request.push(split_arr[1].to_string());
            local_request.push(split_arr[2].to_string());
        }
        SIGNIN_CRED => {
            local_request.push(SIGNIN_CRED.to_string());
        }
        _ => {
            local_request.insert(0, INVALID_RQ.to_string());
        }
    }
    local_request
}


pub async fn auth_cred_login(request: &mut Vec<String>, pool: &PgPool) -> bool {
    let mut head_cpid = request[1].to_owned();
    let cpid = head_cpid.split_off(CPID_HEADER.char_indices().count());
    let mut head_paswd = request[2].to_owned();
    let paswd = head_paswd.split_off(PASSWORD_HEADER.char_indices().count());

    return vaildate_claim(cpid, paswd, pool).await.unwrap();
}


pub async fn login_send_jwt(
    request: &mut Vec<String>,
    pool: &PgPool,
    stream: &mut TcpStream
) -> Result<()> {
    if auth_cred_login(request, pool).await {
        let mut head_cpid = request[1].to_owned();
        let cpid = head_cpid.split_off(CPID_HEADER.char_indices().count());
        let mut head_paswd = request[2].to_owned();
        let paswd = head_paswd.split_off(PASSWORD_HEADER.char_indices().count());

        let claim = Claim{
            cpid,
            paswd,
            exp: exp_gen(),
        };
        let jwt = jwt::create(&claim).await.unwrap();
        stream.write_all(jwt.as_bytes()).await?;
        stream.flush().await?;
        stream.shutdown().await.unwrap();
        let e = Error::from(ErrorKind::NotFound);
        return Err(Error::new(std::io::ErrorKind::NotFound, e))       
    } else {
         stream.write_all(b"404").await.unwrap();
        let _ = stream.flush().await;
        stream.shutdown().await.unwrap();
        
    } 

    Ok(())
}
pub async fn jwt_login(
    request: &mut Vec<String>,
    pool: &PgPool
) -> bool {
    let token = request[1].clone();
    let state: bool = validate_jwt_claim(&token, pool).await;

    return state
} 


