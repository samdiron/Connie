use std::io::{ErrorKind, Result};

use lib_db::types::PgPool;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use crate::common::request::{GET, PACKET_SIZE};
use crate::types::RQM;
use common_lib::path::DATA_DIR;

pub async  fn handle_server_request(
    request: RQM,
    request_type: String,
    stream: &mut TcpStream,
    pool: &PgPool
) -> Result<u8> {
    Ok(0)
}
