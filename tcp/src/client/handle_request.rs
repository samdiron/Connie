use std::io::Result;

use common_lib::log::{debug, info, warn};
use common_lib::tokio::{fs::File, io::{AsyncReadExt, BufReader}, net::TcpStream};
use lib_db::jwt::get_current_timestamp;
use lib_db::sqlite::sqlite_media::SqliteMedia;
use lib_db::types::SqlitePool;
use crate::{
    common::{
        request::{POST, READY_STATUS},
        util::wffb
    }, types::RQM};


/// this function take only the raw request and does not send it you have to send the full request
/// before usin this function and input the raw request GET/POST/ etc
/// the raw request has to constain the path of the file to be posted
pub async fn handle_client_request(
    stream: &mut TcpStream,
    request: RQM,
    host_cpid: String,
    pool: &SqlitePool
) -> Result<u8> {
    let mut status: u8 = 0; 
    if request.header == POST.to_owned() {
        let path: String = request.path.unwrap();
        let f = File::open(&path).await?;
        debug!("post file is open");
        let mut reader = BufReader::new(f);
        let ready = stream.read_u8().await?;
        if ready == READY_STATUS {
            debug!("CLIENT: ready to send file");
            let _size = wffb(stream, request.size as u64 , &mut reader).await?;
            debug!("CLIENT: witing for server confirm ~ ");
            status = stream.read_u8().await?;
            assert_eq!(request.size as usize, _size);
            let date = get_current_timestamp() as i64;
            if status == 0 {
                let sqlitem = SqliteMedia {
                    name: request.name,
                    cpid: request.cpid,
                    type_: request.type_,
                    size: request.size,
                    checksum: request.chcksum,
                    host: host_cpid,
                    date,
                    path,
                };
                SqliteMedia::add_media(sqlitem, pool).await.unwrap();
                println!("CLIENT: file sent succefully; size of file {}",request.size);
                info!("CLIENT: file sent succefully; size of file {}",request.size);
            }else {
                println!("CLIENT:ERROR: server did not recv data the same");
                warn!("CLIENT:ERROR: server did not recv data the same");
            }
        }
        

    } 

    Ok(status)
}



