
use std::io::Result;
use std::process::exit;

use common_lib::log::{
    debug,
    error,
    info,
    warn
};
use common_lib::tokio::{
    fs::File,
    io::{
        AsyncReadExt,
        BufReader,
        AsyncWriteExt,
        BufWriter,
    },
};

use lib_db::types::SqlitePool;
use lib_db::jwt::get_current_timestamp;
use lib_db::media::checksum::{get_fsum, get_size};
use lib_db::sqlite::sqlite_media::{sqlite_delete_media, SqliteMedia};
use tokio::net::TcpStream;

use crate::common::util::core::{raw_rvfs, raw_wffb, raw_wifb};
use crate::types::RQM;
use crate::common::ClientTlsStreams;
use crate::common::util::client::{wffb,rvfs, wifb};
use crate::common::request::{
    GET,
    POST, 
    NO_VAL,
    DELETE,
    SUCCESFUL,
    NOT_FOUND,
    READY_STATUS,
    DATA_NOT_MATCH,
    CLIENT_SIDE_ERR,
    SERVER_SIDE_ERR,
    MEDIA_ALREADY_EXISTS,
};





pub async fn raw_handle_client_request(
    stream: &mut TcpStream,
    request: RQM,
    host_cpid: String,
    check_for_sum: Option<bool>,
    pool: &SqlitePool
) -> Result<u8> {
    let mut status: u8 = 0; 
    match request.header.as_str() {
    POST =>  {
        debug!("serving post request ");
        let path: String = request.path.unwrap();
        let f = File::open(&path).await?;
        debug!("post file is open");
        let mut reader = BufReader::new(f);
        let ready = stream.read_u8().await?;
        if ready == READY_STATUS {
            debug!("ready to send file");
            let verbose = true;
            let _size = raw_wffb(
                    stream,
                    request.size as u64,
                    &mut reader,
                    verbose
            ).await?;
            debug!("witing for server confirm ~ ");
            let checksum = if &request.chcksum == NO_VAL {
                debug!("waiting for server to send checksum");
                let checksum_vector = raw_rvfs(
                        stream
                    ).await?;
                let string_chscksum = String::from_utf8(checksum_vector)
                        .unwrap();
                string_chscksum
            } else {
                request.chcksum
            };
            status = stream.read_u8().await?;
            assert_eq!(request.size as usize, _size);
            let date = get_current_timestamp() as i64;
            if status == 0 {
                let sqlitem = SqliteMedia {
                    name: request.name,
                    cpid: request.cpid,
                    type_: request.type_,
                    size: request.size,
                    checksum,
                    host: host_cpid,
                    date,
                    path,
                };
                SqliteMedia::add_media(sqlitem, pool).await.unwrap();
                info!(
                    "CLIENT: file sent succefully; size of file {}",
                    request.size
                );
            }else {
                warn!("CLIENT:ERROR: server did not recv data the same");
            }
        } else if ready == MEDIA_ALREADY_EXISTS {
            info!("the server has this same media file under your account so it will not be sent");
        }
    }    
    GET => {
        debug!("serving get request ");
        let server_status = stream.read_u8().await?;
        let path = request.path.unwrap();
        let f = File::create(&path).await;
        if f.is_err() {

            stream.write_u8(CLIENT_SIDE_ERR).await?;
            stream.flush().await?;
            status = stream.read_u8().await?;
            assert_ne!(status, 0);
            error!("could not create file:{}",&path);
            exit(CLIENT_SIDE_ERR as i32);
        };
        match server_status {
            READY_STATUS => {
            debug!("GET:request:READY");
                let f = f.unwrap();
                stream.write_u8(READY_STATUS).await?;
                let mut writer = BufWriter::new(f);
                let verbose = true;
                raw_wifb(
                    stream,
                    &mut writer,
                    verbose
                ).await?;
                let local_size = get_size(&path).await?;
                if local_size != request.size {
                    stream.write_u8(DATA_NOT_MATCH).await?;
                    stream.flush().await?;
                    error!("DATA SERVER SENT DOES NOT MATCH WHAT it's supposed to be");
                }else if check_for_sum.unwrap() {
                    debug!("GET:CHECKSUM");
                    let local_sum = get_fsum(&path).await?;
                    if local_sum != request.chcksum {
                        stream.write_u8(DATA_NOT_MATCH).await?;
                        stream.flush().await?;
                        error!("DATA SERVER SENT DOES NOT MATCH WHAT it's supposed to be");
                    }
                        else {
                            stream.write_u8(SUCCESFUL).await?;
                            stream.flush().await?;
                            info!("SUCCESFUL:GET f: {}",&path);
                            assert_eq!(local_sum, request.chcksum)
                        };
                }else {
                    stream.write_u8(SUCCESFUL).await?;
                    stream.flush().await?;
                    info!("SUCCESFUL:GET f: {}",&path);

                }


            }
            SERVER_SIDE_ERR => {
                error!("SERVER SIDE ERROR OCURED code:{SERVER_SIDE_ERR}");
            }
            NOT_FOUND => {
                error!("SERVER SIDE MEDIA NOT FOUND code:{NOT_FOUND}");
            }
                _ => {}
        }
    }
    DELETE => {
        status = stream.read_u8().await?;
        if status == NOT_FOUND {
            error!("server sent not found status; \n did you refreash the list first ?");
        }
        if status == 0 {
            sqlite_delete_media(
                    &host_cpid,
                    &request.cpid,
                    &request.chcksum,
                    pool
            ).await;
        }
    }
       _ => {}
    }

    Ok(status)
}




/// this function take only the raw request and does not send it you have to send the full request
/// before usin this function and input the raw request GET/POST/ etc
/// the raw request has to constain the path of the file to be posted
pub async fn handle_client_request(
    stream: &mut ClientTlsStreams,
    request: RQM,
    host_cpid: String,
    check_for_sum: Option<bool>,
    pool: &SqlitePool
) -> Result<u8> {
    let mut status: u8 = 0; 
    match request.header.as_str() {
    POST =>  {
        debug!("serving post request ");
        let path: String = request.path.unwrap();
        let f = File::open(&path).await?;
        debug!("post file is open");
        let mut reader = BufReader::new(f);
        let ready = stream.read_u8().await?;
        if ready == READY_STATUS {
            debug!("ready to send file");
            let verbose = true;
            let _size = wffb(
                    stream,
                    request.size as u64,
                    &mut reader,
                    verbose
            ).await?;
            debug!("witing for server confirm ~ ");
            let checksum = if &request.chcksum == NO_VAL {
                debug!("waiting for server to send checksum");
                let checksum_vector = rvfs(stream).await?;
                let string_chscksum = String::from_utf8(checksum_vector)
                        .unwrap();
                string_chscksum
            } else {
                request.chcksum
            };
            status = stream.read_u8().await?;
            assert_eq!(request.size as usize, _size);
            let date = get_current_timestamp() as i64;
            if status == 0 {
                let sqlitem = SqliteMedia {
                    name: request.name,
                    cpid: request.cpid,
                    type_: request.type_,
                    size: request.size,
                    checksum,
                    host: host_cpid,
                    date,
                    path,
                };
                SqliteMedia::add_media(sqlitem, pool).await.unwrap();
                info!(
                    "CLIENT: file sent succefully; size of file {}",
                    request.size
                );
            }else {
                warn!("CLIENT:ERROR: server did not recv data the same");
            }
        } else if ready == MEDIA_ALREADY_EXISTS {
            info!("the server has this same media file under your account so it will not be sent");
        }
    }    
    GET => {
        debug!("serving get request ");
        let server_status = stream.read_u8().await?;
        let path = request.path.unwrap();
        let f = File::create(&path).await;
        if f.is_err() {

            stream.write_u8(CLIENT_SIDE_ERR).await?;
            stream.flush().await?;
            status = stream.read_u8().await?;
            assert_ne!(status, 0);
            error!("could not create file:{}",&path);
            exit(CLIENT_SIDE_ERR as i32);
        };
        match server_status {
            READY_STATUS => {
            debug!("GET:request:READY");
                let f = f.unwrap();
                stream.write_u8(READY_STATUS).await?;
                let mut writer = BufWriter::new(f);
                let verbose = true;
                wifb(stream, &mut writer, verbose).await?;
                let local_size = get_size(&path).await?;
                if local_size != request.size {
                    stream.write_u8(DATA_NOT_MATCH).await?;
                    stream.flush().await?;
                    error!("DATA SERVER SENT DOES NOT MATCH WHAT it's supposed to be");
                }else if check_for_sum.unwrap() {
                    debug!("GET:CHECKSUM");
                    let local_sum = get_fsum(&path).await?;
                    if local_sum != request.chcksum {
                        stream.write_u8(DATA_NOT_MATCH).await?;
                        stream.flush().await?;
                        error!("DATA SERVER SENT DOES NOT MATCH WHAT it's supposed to be");
                    }
                        else {
                            stream.write_u8(SUCCESFUL).await?;
                            stream.flush().await?;
                            info!("SUCCESFUL:GET f: {}",&path);
                            assert_eq!(local_sum, request.chcksum)
                        };
                }else {
                    stream.write_u8(SUCCESFUL).await?;
                    stream.flush().await?;
                    info!("SUCCESFUL:GET f: {}",&path);

                }


            }
            SERVER_SIDE_ERR => {
                error!("SERVER SIDE ERROR OCURED code:{SERVER_SIDE_ERR}");
            }
            NOT_FOUND => {
                error!("SERVER SIDE MEDIA NOT FOUND code:{NOT_FOUND}");
            }
                _ => {}
        }
    }
    DELETE => {
        status = stream.read_u8().await?;
        if status == NOT_FOUND {
            error!("server sent not found status; \n did you refreash the list first ?");
        }
        if status == 0 {
            sqlite_delete_media(
                    &host_cpid,
                    &request.cpid,
                    &request.chcksum,
                    pool
            ).await;
        }
    }
       _ => {}
    }

    Ok(status)
}



