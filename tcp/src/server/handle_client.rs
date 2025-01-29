use lib_db::types::PgPool;
use log::info;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpSocket, TcpStream}};
use std::{io::{Read, Result}, net::SocketAddr};
use tokio::time::timeout;

use crate::common::request::split_request;



async fn process_request(stream: &mut TcpStream) -> Result<Vec<String>> {
    let mut buf = vec![0;1080];
    let _size = stream.read(&mut buf).await?;
    let request = String::from_utf8_lossy(&buf);
    let _prequest = split_request(request.to_string());

    Ok(_prequest)
}



pub async fn handle(
    st: (TcpStream, SocketAddr),
    pool: PgPool
) -> Result<()> {
    
    let _p = pool;
    let mut buf = String::new();
    let mut stream = st.0;
    let addr = st.1;
    println!("client: {addr} is being served");
    let duration = tokio::time::Duration::from_millis(700); 
    let len = timeout(duration,stream.read_to_string(&mut buf)).await?;
    if len.unwrap() == 0usize {
        info!("client was droped for not responding fast");
        return Ok(());
    }
    let request_vec = process_request(&mut stream).await?;
    

    Ok(())



}
