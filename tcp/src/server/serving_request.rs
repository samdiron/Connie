use std::io::Result;

use lib_db::types::PgPool;
use tokio::net::TcpSocket;

use crate::common::request::GET;

pub fn handle_server_request(
    raw: String,
    stream: &mut TcpSocket,
    pool: PgPool
) -> Result<u8> {
    if raw.contains(GET) {
     
    }
}
