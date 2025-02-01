use tokio::fs::File;
use std::io::{ErrorKind, Result};

use lib_db::types::PgPool;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use crate::common::request::{GET, PACKET_SIZE};
use common_lib::path::DATA_DIR;

pub async  fn handle_server_request(
    raw: String,
    stream: &mut TcpStream,
    pool: PgPool
) -> Result<u8> {
    if raw.contains(GET) {
        let mut name = String::new();
        stream.read_to_string(&mut name).await?;
        let path = format!("{DATA_DIR}/name");
        let f = File::open(path).await?;
        let mut reader = BufReader::new(f);
        let mut buf = vec![0; PACKET_SIZE as usize];
        loop {
            let _res = reader.read_exact(&mut buf).await;
            if _res.is_ok() {
                stream.write_all(&buf).await?;
                stream.flush().await?
            }
            else if _res.unwrap_err().kind() == ErrorKind::UnexpectedEof {
                stream.write_all(&buf).await?;
                stream.flush().await?;
                break
            }
            else {
                return Ok(1);
            }
        }
    
        return Ok(0)
    }
    Ok(0)
}
