use std::io::Result;

use log::debug;
use tokio::{fs::File, io::{AsyncReadExt, BufReader}, net::TcpStream};

use crate::{common::{request::{POST, READY_STATUS}, util::wffb}, types::RQM};


/// this function take only the raw request and does not send it you have to send the full request
/// before usin this function and input the raw request GET/POST/ etc
/// the raw request has to constain the path of the file to be posted
pub async fn handle_client_request(
    stream: &mut TcpStream,
    request: RQM
) -> Result<u8> {
    let mut status: u8 = 0; 
    if request.header == POST.to_owned() {
        println!("CLIENT: opening file");
        let f = File::open(request.path.unwrap()).await?;
        debug!("post file is open");
        let mut reader = BufReader::new(f);
        let ready = stream.read_u8().await?;
        if ready == READY_STATUS {
            println!("CLIENT: ready to send file");
            let _size = wffb(stream, request.size , &mut reader).await?;
            println!("CLIENT: witing for server confirm ~ ");
            status = stream.read_u8().await?;
            assert_eq!(request.size as usize, _size);
            if status == 0 {
                println!("CLIENT: file sent succefully; size of file {}",request.size);
            }else {
                println!("CLIENT:ERROR: server did not recv data the same");
            }
        }
        

    } 

    Ok(status)
}



