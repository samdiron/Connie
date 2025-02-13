use lib_db::{jwt::validate_jwt_claim, types::{sqlE, PgPool}};
use log::info;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use std::{io::Result, net::SocketAddr, u16};
use tokio::time::timeout;

use crate::{client::handle_request::handle_client_request, common::request::{
    JwtReq, JWT_AUTH, LOGIN_CRED, PACKET_SIZE
}, server::{handle_client, serving_request::handle_server_request}, types::RQM};



    






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
    

    
    Ok(())
}
