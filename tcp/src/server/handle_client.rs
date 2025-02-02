use lib_db::types::PgPool;
use log::info;
use tokio::{io::AsyncReadExt, net::TcpStream};
use std::{io::Result, net::SocketAddr};
use tokio::time::timeout;

use crate::{common::request::{
    get_raw_request, jwt_login, login_send_jwt, split_request, JWT_AUTH, LOGIN_CRED
}, server::serving_request::handle_server_request};


async fn process_request(stream: &mut TcpStream) -> Result<Vec<String>> {
    let mut buf = vec![0;1080];
    let _size = stream.read(&mut buf).await?;
    let request = String::from_utf8_lossy(&buf);
    let _prequest = split_request(request.to_string());

    Ok(_prequest)
}

pub async fn auth_request(
    request: &mut Vec<String>,
    pool: &PgPool,
    stream: &mut TcpStream
) -> Result<bool> {
    let state_of_connection:bool;
    match request[0].as_str() {
        LOGIN_CRED => {
            match login_send_jwt(request, pool, stream).await{
                Ok(..) => {
                    info!("a jwt was sent");
                },
                Err(e) => {
                    eprintln!("error while log in protocol: {e}");
                },
            }
            state_of_connection = false;
        },
        JWT_AUTH => {
            state_of_connection = jwt_login(request, pool).await
        },
        _ => {
            state_of_connection = false;
        }
    }
    

    Ok(state_of_connection)
}

pub async fn handle(
    st: (TcpStream, SocketAddr),
    pool: PgPool
) -> Result<()> {
    
    let mut buf = String::new();
    let mut stream = st.0;
    let addr = st.1;
    println!("client: {addr} is being served");
    let duration = tokio::time::Duration::from_millis(700); 
    let len = timeout(duration,stream.read_to_string(&mut buf)).await?;
    if 0usize == len.unwrap() {
        info!("client was droped for not responding fast");
        return Ok(());
    }

    let mut request_vec = process_request(&mut stream).await?;
    let state_of_connection = auth_request(
        &mut request_vec,
        &pool,
        &mut stream
    ).await.unwrap();
    if false == state_of_connection {
        drop(stream);
        return Ok(());
    } else {
        let raw = get_raw_request(&mut request_vec).unwrap();

        match handle_server_request(
            raw,
            &mut stream,
            pool
        ).await {
            Ok(state) => {
                if state == 0 {
                    println!("a request was handled");
                }
                else {
                    println!("error a request returned an error ")
                }
            }
            Err(e) => {
                eprintln!("an error while serving a request: {:?} ", e)
            }
        }
    }
    
    Ok(())
}
