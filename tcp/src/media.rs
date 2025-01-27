use lib_db::types::PgPool;
use tokio::{io::AsyncWriteExt, net::TcpStream};
use std::{fs::File, io::{BufReader, Read, Result}};
use crate::common::request::PACKET_SIZE;




pub async fn receiver(stream: &mut TcpStream, pool: &PgPool) -> Result<()> {
    Ok(())
}



pub async fn sender(stream: &mut TcpStream, f: File) -> Result<()> {
    let mut reader = BufReader::new(f);
    let mut buffer = vec![0; PACKET_SIZE as usize];
    loop {
        let _size = reader.read(&mut buffer)?;
        if _size == 0 {
            break;
        }
        else {
            stream.write_all(&buffer).await?;
        }
    }

    Ok(())
}
