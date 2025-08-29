
use lib_db::media;
use lib_db::types::PgPool;
use lib_db::user::user_struct;
use lib_db::media::fetch::Smedia;
use lib_db::server::host::fetch_host_public_files;

use lib_db::sqlite::{
    sqlite_host::{self, SqliteHost},
    sqlite_user::{ShortUser, SqliteUser}
};

use common_lib::bincode;
use common_lib::log::{debug, info, warn};

use tokio::{
    net::TcpStream,
    io::{AsyncReadExt, AsyncWriteExt},
};

use tokio_rustls::server::TlsStream;

use std::net::SocketAddr;
use std::time::{Duration, SystemTime};
use std::io::{
    Error,
    Result,
    ErrorKind,
};

use crate::common::util::core::{raw_read_stream, raw_rvfs, raw_wvts};
use crate::common::{
    handshakes,
    request::{
        FETCH,
        JWT_AUTH,
        LOGIN_CRED,
        SIGNIN_CRED,
        UNAUTHORIZED
    },
    util::server::{
        read_stream,
        rvfs,
        wvts,
    }
};

use crate::server::request_handles::UNMATCHED_CPID;
use crate::server::runtime as rt;
use rt::logs::client_log;
use rt::statics::ALL_REQUESTS;
use rt::generate_log_templates::ClientErrorMsgLog;

use crate::server::req_format::{
    Chead,
    JwtReq,
    LoginReq
};
use crate::server::request_handles::serving_request::{
    handle_server_request,
    raw_handle_server_request,
};





async fn login_create_jwt(
    request: LoginReq,
    host_cpid: &String,
    pool: &PgPool,
) -> Result<String> {
    let is_val = request.validate(host_cpid, pool).await;
    if is_val.is_ok() && is_val.unwrap() == true {
            let jwt = request.token_gen().await.unwrap();
            return Ok(jwt)
    }else {
        
        let e = Error::new(ErrorKind::NotFound, "user not found");
        return Err(e)
    }
}



pub async fn raw_handle(
    st: (TcpStream, SocketAddr),
    pool: PgPool,
    allow_new_users: bool,
    sqlite_host: SqliteHost,
) -> Result<(u8, Option<ClientErrorMsgLog>)> {
    
    let mut stream = st.0;
    let addr = st.1;
    // this function assures the client that is 
    // in the correct addres and send the pub ip of
    // the server and if the if the client is in the correct addres it will return 0 else 1
    let is_correct_addres = handshakes::raw_server_handshake(
        &mut stream,
        &sqlite_host
    ).await?;
    if is_correct_addres == 1 {
        debug!("a client was lost");
        return Ok((1, None))
    }else if is_correct_addres == SIGNIN_CRED {
        
        debug!("SERVER: signin request");
        if allow_new_users {
            stream.write_u8(0).await?;
            stream.flush().await?;
            let server_vector = sqlite_host::SqliteHost::sz(
                sqlite_host.clone()
            ).unwrap();
            debug!("server vector size: {}",server_vector.len());

            raw_wvts(
                &mut stream,
                server_vector
            ).await?;

            let confirm = stream.read_u8().await?;
            if confirm == 0 { 
                let vector = raw_rvfs(
                    &mut stream,
                ).await?;
                let short_user = ShortUser::dz(vector).unwrap();
                let user = user_struct::User {
                    cpid: String::new(),
                    name: short_user.name,
                    username: short_user.username,
                    password: short_user.password,
                    email: short_user.email,
                    host: sqlite_host.cpid,
                };
                let _user = user.create(&pool).await.unwrap();
                let sqlite_user = SqliteUser {
                    cpid: _user.cpid,
                    name: _user.name,
                    host: _user.host,
                    email: _user.email,
                    usrname: _user.username,
                };
                let user_vector = sqlite_user.sz().unwrap();
                debug!("user vector size: {}",user_vector.len());
                raw_wvts(&mut stream, user_vector).await?;

                stream.write_u8(0).await?;
                stream.flush().await?;
            } else {
                warn!("a user tried to signup then declind");
            }
        } else {
            stream.write_u8(UNAUTHORIZED).await?;
            stream.flush().await?;
            let _ = client_log(
                addr.ip(),
                &"NULL".to_string(),
                &"SIGNUP".to_string(),
                UNAUTHORIZED
            ).await?;
        }
        stream.shutdown().await?;
    } else {
    let auth_type = stream.read_u8().await?;
    info!("SERVER: C{addr} will auth with {auth_type}");
    match auth_type {
        JWT_AUTH => {
            debug!("SERVER: jwt auth request");
            let buf = raw_read_stream(&mut stream, 600).await?;
            if buf.is_empty() {
                debug!("an empty request was sent");
            }
            debug!("SERVER: recved request with size: {}", buf.len());
            
            let jwtreq = JwtReq::dz(buf).expect("could not unwrap struct");
            let ( is_valid, current_client_cpid )= jwtreq.validate(
                    &sqlite_host.cpid,
                    &pool
                ).await.unwrap();
            if  is_valid && (
                current_client_cpid == jwtreq.request.cpid ||
                sqlite_host.cpid == jwtreq.request.cpid 
            ) {
                stream.write_u8(0).await?;
                debug!("SERVER: valid jwt login");
                let status = raw_handle_server_request(
                    jwtreq.request,
                    &mut stream,
                    &sqlite_host.cpid,
                    &pool
                ).await?;
                if status == 0 {
                    info!("a request was handled succsefully");
                }
                stream.write_u8(status).await?;
                stream.flush().await?;

                drop(stream);
            } else if current_client_cpid != jwtreq.request.cpid {
                    let client_ip = addr.ip().to_string();
                    let timestamp = SystemTime::now();
                let err = ClientErrorMsgLog {
                    client_jwt_cpid: current_client_cpid,
                    client_request_cpid: jwtreq.request.cpid,
                    client_ip,
                    timestamp,
                    sev: 1,
                };
                return Ok( ( UNMATCHED_CPID, Some(err) ) );
            }
            else {
                let dur = Duration::from_secs(3);
                std::thread::sleep(dur);
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
            let cpid = request.cpid.clone();
            let is_jwt = login_create_jwt(
                    request,
                    &sqlite_host.cpid,
                    &pool
            ).await;
            if is_jwt.is_ok() {
                let jwt = is_jwt?;
                debug!("SERVER: login about to compleate");
                stream.write_u8(0).await?;
                stream.write_all(jwt.as_bytes()).await?;
                debug!("sent: {} bytes", jwt.len());
                stream.flush().await?;
                let confirm = stream.read_u8().await?;
                if confirm  == 0 { 
                    debug!("SERVER: client login succsefully")
                };
                drop(stream);
                
                debug!("SERVER: client logged in succsefully ");
                let _ = client_log(addr.ip(), &cpid, "LOGIN", 0).await?;

            } else {
                debug!("a login with res code {UNAUTHORIZED}");
                debug!("SERVER: login faild");
                stream.write_u8(UNAUTHORIZED).await?;
                stream.flush().await?;
                let _ = client_log(
                        addr.ip(), 
                        &cpid, 
                        "LOGIN", 
                        UNAUTHORIZED
                ).await?;

            }

        }
        FETCH => {
            debug!("SERVER: fetch request");

            let mut buf = vec![0;600];
            let _size = stream.read(&mut buf).await?;
            let request = Chead::dz(buf).expect("could not deserialze");
            let (is_val, current_client_cpid ) = request.validate(
                    &sqlite_host.cpid,
                    &pool
            ).await.unwrap();
            if is_val && (request.cpid == current_client_cpid) {

                let data: Vec<Smedia> = media::fetch::get_user_files(
                    request.cpid,
                    sqlite_host.cpid.clone(),
                    &pool
                ).await.unwrap();
                stream.write_u16(data.len() as u16).await?;
                for d in data {
                    let vec = bincode::serialize(&d).unwrap();
                    let s = raw_wvts(&mut stream, vec).await?;
                    assert_eq!(s, 0);

                }

                let pub_file = stream.read_u8().await?;
                if pub_file == 1 {
                    
                    let data: Vec<Smedia> = fetch_host_public_files(
                        &sqlite_host,
                        &pool
                    ).await.unwrap();
                    stream.write_u16(data.len() as u16).await?;
                    for d in data {
                        let vec = bincode::serialize(&d).unwrap();
                        let s = raw_wvts(&mut stream, vec).await?;
                        assert_eq!(s, 0);

                }};

                let _ = client_log(
                        addr.ip(),
                        &current_client_cpid,
                        "FETCH",
                        0
                ).await?;

                
            } else if &request.cpid != &current_client_cpid {
                
                let client_ip = stream
                        .peer_addr()
                        .unwrap()
                        .ip()
                        .to_string();
                let timestamp = SystemTime::now();
                let err = ClientErrorMsgLog {
                    client_jwt_cpid: current_client_cpid,
                    client_request_cpid: request.cpid,
                    client_ip,
                    timestamp,
                    sev: 1,
                };
                let _ = client_log(
                        addr.ip(),
                        &"NULL".to_string(),
                        &"FETCH".to_string(),
                        UNMATCHED_CPID
                    ).await?;
                return Ok( ( UNMATCHED_CPID, Some(err) ) );
                
            }
        }
        _=> {info!("client sent a invalid auth header: {auth_type}")}
    }

    } 
    Ok( (0, None) )
}

pub async fn handle(
    st: (TlsStream<TcpStream>, SocketAddr),
    pool: PgPool,
    allow_new_users: bool,
    sqlite_host: SqliteHost,
) -> Result<(u8, Option<ClientErrorMsgLog>)> {
    
    let mut stream = st.0;
    let addr = st.1;
    // this function assures the client that is 
    // the correct addres and send the pub ip of
    // the server and if the if the client is in the correct addres it will return 0 else 1
    let is_correct_addres = handshakes::server(
        &mut stream,
        &sqlite_host
    ).await?;
    if is_correct_addres == 1 {
        debug!("a client was lost");
        return Ok((1, None))
    }else if is_correct_addres == SIGNIN_CRED {
        
        debug!("SERVER: signin request");
        if allow_new_users {
            stream.write_u8(0).await?;
            stream.flush().await?;
            let server_vector = sqlite_host::SqliteHost::sz(
                sqlite_host.clone()
            ).unwrap();
            debug!("server vector size: {}",server_vector.len());
            wvts(&mut stream, server_vector).await?;
            let confirm = stream.read_u8().await?;
            if confirm == 0 { 
                let vector = rvfs(&mut stream).await?;
                let short_user = ShortUser::dz(vector).unwrap();
                let user = user_struct::User {
                    cpid: String::new(),
                    name: short_user.name,
                    username: short_user.username,
                    password: short_user.password,
                    email: short_user.email,
                    host: sqlite_host.cpid,
                };
                let _user = user.create(&pool).await.unwrap();
                let sqlite_user = SqliteUser {
                    cpid: _user.cpid.clone(),
                    name: _user.name,
                    host: _user.host,
                    email: _user.email,
                    usrname: _user.username,
                };
                let user_vector = sqlite_user.sz().unwrap();
                debug!("user vector size: {}",user_vector.len());
                wvts(&mut stream, user_vector).await?;

                stream.write_u8(0).await?;
                stream.flush().await?;
            } else {
                warn!("a user tried to signup then declind");
            }
        } else {
            let _ = client_log(
                addr.ip(),
                &"NULL".to_string(),
                &"SIGNUP".to_string(),
                UNAUTHORIZED
            ).await?;
            stream.write_u8(UNAUTHORIZED).await?;
            stream.flush().await?;
        }
        stream.shutdown().await?;
    } else {
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

            let rqm_admin_clone = jwtreq.request.clone();
            let ( is_valid, current_client_cpid )= jwtreq.validate(
                    &sqlite_host.cpid,
                    &pool
                ).await.unwrap();
            if  is_valid && current_client_cpid == jwtreq.request.cpid ||
                sqlite_host.cpid == jwtreq.request.cpid {
                stream.write_u8(0).await?;
                debug!("SERVER: valid jwt login");
                let status = handle_server_request(
                    jwtreq.request,
                    &mut stream,
                    addr.ip(),
                    &sqlite_host.cpid,
                    &pool
                ).await?;
                if status == 0 {
                    info!("a request was handled succsefully");
                }
                stream.write_u8(status).await?;
                stream.flush().await?;

                drop(stream);
            } else if current_client_cpid != jwtreq.request.cpid {
                    let client_ip = addr.ip().to_string();
                    let timestamp = SystemTime::now();
                let err = ClientErrorMsgLog {
                    client_jwt_cpid: current_client_cpid,
                    client_request_cpid: jwtreq.request.cpid,
                    client_ip,
                    timestamp,
                    sev: 1,
                };
                return Ok( ( UNMATCHED_CPID, Some(err) ) );
            }
            else {
                let dur = Duration::from_secs(3);
                std::thread::sleep(dur);
                stream.write_u8(UNAUTHORIZED).await?;
                debug!("SERVER: jwt auth invalid");

            }
            match ALL_REQUESTS.lock() {
                Ok(mut val) => {
                    val.push(rqm_admin_clone);
                }
                Err(e) => {
                        debug!(
                            "couldn't lock ALL_REQUESTS to push admin copy; msg = {}",
                            e.to_string()
                        )
                    }

            }

        }
        
        LOGIN_CRED => {
            debug!("SERVER: login request");
            let mut buf = vec![0; 300];
            let size = stream.read(&mut buf).await?;
            debug!("request size: {}",size);
            let request = LoginReq::dz(buf).expect("could not deserialze");
            let cpid = request.cpid.clone();
            let is_jwt = login_create_jwt(
                    request,
                    &sqlite_host.cpid,
                    &pool
            ).await;
            if is_jwt.is_ok() {
                let jwt = is_jwt?;
                debug!("SERVER: login about to compleate");
                stream.write_u8(0).await?;
                stream.write_all(jwt.as_bytes()).await?;
                debug!("sent: {} bytes", jwt.len());
                stream.flush().await?;
                let confirm = stream.read_u8().await?;
                if confirm  == 0 { 
                    debug!("SERVER: client login succsefully")
                };
                drop(stream);
                
                debug!("SERVER: client logged in succsefully ");

                let _ = client_log(addr.ip(), &cpid, "LOGIN", 0).await?;
            } else {
                debug!("a login with res code {UNAUTHORIZED}");
                debug!("SERVER: login faild");
                stream.write_u8(UNAUTHORIZED).await?;
                stream.flush().await?;
                let _ = client_log(
                        addr.ip(),
                        &cpid,
                        "LOGIN",
                        UNAUTHORIZED
                ).await?;

            }

        }
        FETCH => {
            debug!("SERVER: fetch request");

            let mut buf = vec![0;600];
            let _size = stream.read(&mut buf).await?;
            let request = Chead::dz(buf).expect("could not deserialze");
            let (is_val, current_client_cpid ) = request.validate(
                    &sqlite_host.cpid, &pool
                ).await.unwrap();
            if is_val && (request.cpid == current_client_cpid) {

                let data: Vec<Smedia> = media::fetch::get_user_files(
                    request.cpid.clone(),
                    sqlite_host.cpid.clone(),
                    &pool
                ).await.unwrap();
                stream.write_u16(data.len() as u16).await?;
                for d in data {
                    let vec = bincode::serialize(&d).unwrap();
                    let s = wvts(&mut stream, vec).await?;
                    assert_eq!(s, 0);

                }

                let pub_file = stream.read_u8().await?;
                if pub_file == 1 {
                    
                    let data: Vec<Smedia> = fetch_host_public_files(
                        &sqlite_host,
                        &pool
                    ).await.unwrap();
                    stream.write_u16(data.len() as u16).await?;
                    for d in data {
                        let vec = bincode::serialize(&d).unwrap();
                        let s = wvts(&mut stream, vec).await?;
                        assert_eq!(s, 0);

                };
                let _ = client_log(
                            addr.ip(),
                            &current_client_cpid,
                            "FETCH",
                            0
                        ).await?;

            } else if &request.cpid != &current_client_cpid {
                
                let client_ip = stream.into_inner().0
                        .peer_addr()
                        .unwrap()
                        .ip()
                        .to_string();
                let timestamp = SystemTime::now();
                let err = ClientErrorMsgLog {
                    client_jwt_cpid: current_client_cpid,
                    client_request_cpid: request.cpid,
                    client_ip,
                    timestamp,
                    sev: 1,
                };
                let _ = client_log(
                            addr.ip(),
                            &"NULL".to_string(),
                            &"FETCH".to_string(),
                            UNMATCHED_CPID
                        ).await?;
                return Ok( ( UNMATCHED_CPID, Some(err) ) );
                
            }}
        }
        _=> {info!("client sent a invalid auth header: {auth_type}")}
    }

    } 
    Ok( (0, None) )
}
