use lib_db::{
    media::{self, fetch::Smedia},
    sqlite::{
        sqlite_host::{self, SqliteHost},
        sqlite_user::{ShortUser, SqliteUser}
    }, types::PgPool, user::user_struct
};
use common_lib::{gethostname::gethostname, log::{debug, info, warn}};
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
            FETCH, JWT_AUTH, LOGIN_CRED, SIGNIN_CRED, UNAUTHORIZED
        },
        util::{read_stream, rvfs, wvts}
    }, server::{
        req_format::{
            Chead,
            JwtReq,
            LoginReq
        }, serving_request::handle_server_request

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
    pool: PgPool,
    allow_new_users: bool,
    sqlite_host: SqliteHost,
) -> Result<()> {
    
    let mut stream = st.0;
    let addr = st.1;
    let auth_type = stream.read_u8().await?;
    info!("SERVER: C{addr} will auth with {auth_type}");
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
                stream.write_u8(0).await?;
                debug!("SERVER: valid jwt login");
                let status = handle_server_request(jwtreq.request, &mut stream, &sqlite_host.cpid, &pool).await?;
                if status == 0 {
                    info!("a request was handled succsefully");
                }
                stream.write_u8(status).await?;
                stream.flush().await?;

                drop(stream);
            }
            else {
                stream.write_u8(UNAUTHORIZED).await?;
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
                debug!("sent: {} bytes", jwt.len());
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
        SIGNIN_CRED => {
            debug!("SERVER: signin request");
            if allow_new_users {
                stream.write_u8(0).await?;
                stream.flush().await?;
                let server_vector = sqlite_host::SqliteHost::sz(sqlite_host).unwrap();
                debug!("server vector size: {}",server_vector.len());
                wvts(&mut stream, server_vector).await?;
                let confirm = stream.read_u8().await?;
                if confirm == 0 { 
                    let vector = rvfs(&mut stream).await?;
                    let short_user = ShortUser::dz(vector).unwrap();
                    let host = gethostname().to_str().unwrap().to_string();
                    let user = user_struct::User {
                        cpid: String::new(),
                        name: short_user.name,
                        username: short_user.username,
                        password: short_user.password,
                        email: short_user.email,
                        host,
                    };
                    let _user = user.create(&pool).await.unwrap();
                    let sqlite_user = SqliteUser {
                        cpid: _user.cpid,
                        name: _user.name,
                        host: _user.host,
                        email: _user.email,
                        paswd: _user.password,
                        usrname: _user.username,
                    };
                    let user_vector = sqlite_user.sz().unwrap();
                    debug!("user vector size: {}",user_vector.len());
                    wvts(&mut stream, user_vector).await?;

                    stream.write_u8(0).await?;
                    stream.flush().await?;
                } else {warn!("a user tried to signup then declind")}
            } else {
                stream.write_u8(UNAUTHORIZED).await?;
                stream.flush().await?;
            }

        }
        FETCH => {
            debug!("SERVER: fetch request");

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
        _=> {info!("client sent a invalid auth header: {auth_type}")}
    }

    
    Ok(())
}
