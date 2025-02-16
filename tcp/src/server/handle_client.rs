use lib_db::{jwt::{self, exp_gen, validate_jwt_claim}, types::PgPool, user::user_struct::validate_claim_wcpid};
use lib_db::jwt::Claim;
use log::info;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use std::{io::{Error, ErrorKind, Result}, net::SocketAddr};
use tokio::time::timeout;

use crate::{common::{request::{
    JwtReq, LoginReq, JWT_AUTH, LOGIN_CRED, UNAUTHORIZED
}, util::read_stream}, server::serving_request::handle_server_request};



    


async fn c_jwtreq(
    stream: &mut TcpStream,
    pool: &PgPool,
    request: JwtReq
) -> Result<u8> {
    let is_valid = validate_jwt_claim(&request.jwt, pool).await;
    if is_valid {
        let status = handle_server_request(request.request, stream, pool).await?;
        Ok(status)
    }else {
        Ok(1)
    }
}

async fn login_create_jwt(pool: &PgPool, request: LoginReq) -> Result<String> {
    let exp = exp_gen(); 
    let claim = Claim {
        cpid: request.cpid,
        paswd: request.paswd,
        exp

    };
    let is_val = validate_claim_wcpid(claim.cpid.clone(), claim.paswd.clone(), pool).await.unwrap();
    if is_val {
        let jwt = jwt::create(&claim).await.unwrap();
        println!("jwt was created: {jwt}");
        return Ok(jwt)
    }else {
        
        println!("invalid login request");
        let e = Error::new(ErrorKind::NotFound, "user not found");
        return Err(e)
    }
}

pub async fn handle(
    st: (TcpStream, SocketAddr),
    pool: PgPool
) -> Result<()> {
    
    let mut stream = st.0;
    let addr = st.1;
    
    println!("client: {addr} is being served");
    let duration = tokio::time::Duration::from_secs(10);
    let _auth_type = timeout(duration,stream.read_u8()).await?;
    let auth_type = _auth_type?;

    match auth_type {
        JWT_AUTH => {
            let buf = read_stream(&mut stream, 400).await?;
            let request = JwtReq::dz(buf).expect("could not unwrap struct");
            let status  = c_jwtreq(&mut stream, &pool, request).await?;
            if status == 0 {
                info!("a request was handled succsefully");
            }
            drop(stream);

        }
        
        LOGIN_CRED => {
            let buf = read_stream(&mut stream, 400).await?;
            let request = LoginReq::dz(buf).expect("could not deserialze");
            let is_jwt = login_create_jwt(&pool, request).await;
            if is_jwt.is_ok() {
                let jwt = is_jwt?;
                stream.write_all(jwt.as_bytes()).await?;
                stream.flush().await?;
                let confirm = stream.read_u8().await?;
                if confirm  == 0 {println!("client login")};
                drop(stream);
            } else {
                stream.write_u8(UNAUTHORIZED).await?;
                stream.flush().await?;


            }




        }
        _=> {}
    }

    
    Ok(())
}
