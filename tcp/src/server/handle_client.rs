use lib_db::{media::{self, fetch::Smedia}, types::PgPool};
use common_lib::log::{debug, info};
use common_lib::tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream
};
use common_lib::bincode;
use std::{
    io::{Error, ErrorKind, Result},
    net::SocketAddr
};

use crate::{
    common::{
        request::{
            FETCH, JWT_AUTH, LOGIN_CRED, UNAUTHORIZED
        },
        util::{read_stream, wvts}
    },
    server::{
        req_format::{Chead, JwtReq, LoginReq}, serving_request::handle_server_request

    }
};

    



async fn login_create_jwt(pool: &PgPool, request: LoginReq) -> Result<String> {
    let is_val = request.validate(pool).await;
    if is_val.is_ok() && is_val.unwrap() == true {
            let jwt = request.token_gen().await.unwrap();
            return Ok(jwt)
    }else {
        
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
    let auth_type = stream.read_u8().await?;
    println!("SERVER: C{addr} will auth with {auth_type}");
    match auth_type {
        JWT_AUTH => {
            debug!("SERVER: jwt auth request");
            let buf = read_stream(&mut stream, 600).await?;
            if buf.is_empty() {
                debug!("an empty request was sent");
            }
            debug!("SERVER: recved request with size: {}", buf.len());
            
            let jwtreq = JwtReq::dz(buf).expect("could not unwrap struct");
            let is_valid = jwtreq.validate(&pool).await.unwrap();
            if  is_valid {
                debug!("SERVER: valid jwt login");
                let status = handle_server_request(jwtreq.request, &mut stream, &pool).await?;
                if status == 0 {
                    info!("a request was handled succsefully");
                }
                drop(stream);
            }
            else {

                println!("SERVER: jwt auth invalid");
                debug!("SERVER: jwt auth invalid");

            }

        }
        
        LOGIN_CRED => {
            debug!("SERVER: login request");
            let mut buf = vec![0; 300];
            let size = stream.read(&mut buf).await?;
            debug!("request size: {}",size);
            let request = LoginReq::dz(buf).expect("could not deserialze");
            let is_jwt = login_create_jwt(&pool, request).await;
            if is_jwt.is_ok() {
                let jwt = is_jwt?;
                debug!("SERVER: login about to compleate");
                stream.write_u8(0).await?;
                stream.write_all(jwt.as_bytes()).await?;
                println!("sent: {} bytes", jwt.len());
                stream.flush().await?;
                let confirm = stream.read_u8().await?;
                if confirm  == 0 { debug!("SERVER: client login succsefully")};
                drop(stream);
                
                debug!("SERVER: client logged in succsefully ");

            } else {
                debug!("a login with res code {UNAUTHORIZED}");
                debug!("SERVER: login faild");
                stream.write_u8(UNAUTHORIZED).await?;
                stream.flush().await?;

            }

        } 
        FETCH => {
            println!("SERVER: fetch request");
            let mut buf = vec![0;600];
            let _size = stream.read(&mut buf).await?;
            let request = Chead::dz(buf).expect("could not deserialze");
            let is_val = request.validate(&pool).await.unwrap();
            if is_val {

                let data: Vec<Smedia> = media::fetch::get_user_files(request.cpid, &pool).await.unwrap();
                stream.write_u16(data.len() as u16).await?;
                for d in data {
                    let vec = bincode::serialize(&d).unwrap();
                    let s = wvts(&mut stream, vec).await?;
                    assert_eq!(s, 0);

                }
            }
        }
        _=> {debug!("client sent a invalid auth header: {auth_type}")}
    }

    
    Ok(())
}
