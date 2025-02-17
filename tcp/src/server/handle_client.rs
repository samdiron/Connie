use lib_db::{jwt::{self, exp_gen, validate_jwt_claim}, types::PgPool, user::user_struct::validate_claim_wcpid};
use lib_db::jwt::Claim;
use log::{debug, info};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use std::{io::{Error, ErrorKind, Result}, net::SocketAddr};

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
    println!("SERVER: trying to login client now");
    let exp = exp_gen(); 
    let claim = Claim {
        cpid: request.cpid.clone(),
        paswd: request.paswd.clone(),
        exp

    };
    let is_val = validate_claim_wcpid(request.name, request.paswd, pool).await;
    if is_val.is_ok() {
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
    let auth_type = stream.read_u8().await?;
    println!("SERVER: C{addr} will auth with {auth_type}");
    match auth_type {
        JWT_AUTH => {
            debug!("SERVER: jwt auth request");
            let buf = read_stream(&mut stream, 400).await?;
            if buf.is_empty() {
                println!("an empty request was sent");
                debug!("an empty request was sent");
            }
            let request = JwtReq::dz(buf).expect("could not unwrap struct");
            let status  = c_jwtreq(&mut stream, &pool, request).await?;
            if status == 0 {
                info!("a request was handled succsefully");
            }
            drop(stream);

        }
        
        LOGIN_CRED => {
            debug!("SERVER: login request");
            println!("SERVER: login request");
            let mut buf = vec![0; 300];
            let size = stream.read(&mut buf).await?;
            debug!("request size: {}",size);
            let request = LoginReq::dz(buf).expect("could not deserialze");
            let is_jwt = login_create_jwt(&pool, request).await;
            println!("SERVER: jwt? created");
            if is_jwt.is_ok() {
                let jwt = is_jwt?;
                debug!("SERVER: login about to compleate");
                println!("SERVER: login about to compleate");
                stream.write_u8(0).await?;
                stream.write_all(jwt.as_bytes()).await?;
                println!("sent: {} bytes", jwt.len());
                stream.flush().await?;
                let confirm = stream.read_u8().await?;
                if confirm  == 0 { debug!("SERVER: client login succsefully")};
                drop(stream);
                
                debug!("SERVER: client logged in succsefully ");
                println!("SERVER: client logged in succsefully ");

            } else {
                debug!("a login with res code {UNAUTHORIZED}");
                debug!("SERVER: login faild");
                println!("SERVER: login faild");
                stream.write_u8(UNAUTHORIZED).await?;
                stream.flush().await?;

            }

        }
        _=> {debug!("client sent a invalid auth header: {auth_type}")}
    }

    
    Ok(())
}
