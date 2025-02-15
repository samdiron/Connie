use std::{env, io::Result};

use common_lib::path::DATA_DIR;
use lib_db::types::PgPool;
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter}, net::TcpStream};

use crate::{common::{request::{DELETE, GET, PACKET_SIZE, POST, READY_STATUS, SUCCESFUL}, util::wffb}, types::RQM};


/// this function take only the raw request and does not send it you have to send the full request
/// before usin this function and input the raw request GET/POST/ etc
/// the raw request has to constain the path of the file to be posted
pub async fn handle_client_request(
    stream: &mut TcpStream,
    request: RQM
) -> Result<u8> {
    let mut status: u8 = 0; 
    if request.header == POST.to_owned() {
        let f = File::open(request.path.unwrap()).await?;
        let mut reader = BufReader::new(f);
        let ready = stream.read_u8().await?;
        if ready == READY_STATUS {
            let _status_size = wffb(stream, request.size , &mut reader).await?;
            status = _status_size.0;
            assert_eq!(request.size as usize, _status_size.1);
        }
        

    } 

    
    Ok(status)
}



