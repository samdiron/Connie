
use std::io::Result;
use std::net::IpAddr;
use std::str::FromStr;
use std::path::PathBuf;
use std::fs::{metadata, remove_file};

use lib_db::types::PgPool;
use lib_db::media::checksum::{get_fsum, get_size};
use lib_db::media::media::{
    Media,
    delete_media,
    check_if_media_exist,
    check_if_media_exist_wchecksum,
};

use common_lib as cl;
use cl::path::DATA_DIR;
use cl::tokio::fs::File;
use cl::tokio::io::BufWriter;
use cl::tokio::net::TcpStream;
use cl::tokio::io::AsyncWriteExt;
use cl::log::{debug, error, info};
use cl::tokio::io::{AsyncReadExt, BufReader};

use crate::types::RQM;
use crate::common::ServerTlsStreams;
use crate::common::util::server::{wffb, wifb, wvts};
use crate::common::request::{
    GET,
    POST,
    DELETE,
    NO_VAL,
    NOT_FOUND,
    SUCCESFUL,
    READY_STATUS,
    DATA_NOT_MATCH,
    SERVER_SIDE_ERR,
    MEDIA_ALREADY_EXISTS,
};

use crate::server::runtime::logs::client_log;



const POST_HEADER: &str = "POST";
const GET_HEADER: &str = "GET";
const DELETE_HEADER: &str = "DELETE";

pub async fn raw_handle_server_request(
    request: RQM,
    stream: &mut TcpStream,
    host_cpid: &String,
    pool: &PgPool
) -> Result<u8> {
    let ip = stream.peer_addr()?.ip();
    let cpid = request.cpid.clone();
    match request.header.as_str() {
        POST => {
            if check_if_media_exist(
                &request.cpid,
                host_cpid,
                &request.name,
                &request.type_,
                request.size,
                pool
            ).await {
                stream.write_u8(MEDIA_ALREADY_EXISTS).await?;
                stream.flush().await?;
                
            } else {
            debug!("SERVER: handling( client post request )");
            let mut path = PathBuf::new();
            path.push(DATA_DIR);
            let name = lib_db::fncs::random_string(16);
            path.push(name);
            let f = File::create_new(&path).await?;
            info!("SERVER: created {:#?} ",&path);
            let spath = path.to_str().unwrap();
            let mut writer = BufWriter::new(f);
            stream.write_u8(READY_STATUS).await?;
            let _size = wifb(
                    None,
                    Some(stream),
                    &mut writer,
                ).await?;
            let local_sum = get_fsum(spath).await?;
            let local_size = get_size(spath).await?;
            if &request.chcksum == NO_VAL {
                debug!("a client sent a file with no checksum");
                wvts(None, Some(stream), local_sum.as_bytes().to_vec())
                        .await
                        .unwrap();
            }else if request.chcksum != local_sum {
                stream.write_u8(DATA_NOT_MATCH).await?;
                stream.flush().await?;
            };
            if request.size != local_size {
                stream.write_u8(DATA_NOT_MATCH).await?;
                stream.flush().await?;
            };
            let media = Media {
                name: request.name,
                cpid: request.cpid,
                path: spath.to_owned(),
                checksum: local_sum,
                in_host: host_cpid.clone(),
                type_: request.type_,
                size: local_size,
            };
            let _res = media.post(pool).await.unwrap();
            assert_eq!(_res, 0);
            let _ = client_log(ip, &cpid, POST_HEADER, 0).await?;
            }
        }
        GET => {
            if check_if_media_exist(
                &request.cpid,
                host_cpid,
                &request.name,
                &request.type_,
                request.size,
                pool
            ).await { 
                let media = Media::get(
                    host_cpid,
                    &request.cpid,
                    &request.chcksum,
                    pool
                ).await.unwrap();
                let path = PathBuf::from(&media.path);
                assert_eq!(true, path.exists());
                let metadata = metadata(&path);
                let f = File::open(&path).await?;
                let size = if metadata.is_ok() {
                    let size = metadata.unwrap().len();
                    assert_eq!(size as i64, request.size);
                    size 
                } else  {
                    error!(
                        "could not open file: {}",
                        media.path
                    );
                    0

                };
                if size == 0 {
                    stream.write_u8(SERVER_SIDE_ERR).await?;
                    stream.flush().await?;
                    let _ = client_log(ip, &cpid, GET_HEADER, SERVER_SIDE_ERR).await?;
                } else {
                    stream.write_u8(READY_STATUS).await?;
                    stream.flush().await?;
                    let ready = stream.read_u8().await?;
                    if ready == READY_STATUS {
                        let mut reader = BufReader::new(f);
                        wffb(
                            None,
                            Some(stream),
                            size,
                            &mut reader,
                        ).await?;
                        let confirm = stream.read_u8().await?;
                        if confirm == SUCCESFUL {
                            info!("SUCCESFUL:GET");
                        } else {
                            let _ = client_log(ip, &cpid, GET_HEADER, 1).await?;
                            debug!("UNSUCCESFUL:GET");
                            return Ok(1)
                        }
                        let _ = client_log(ip, &cpid, GET_HEADER, 0).await?;
                    }
                }
            } else {
                stream.write_u8(NOT_FOUND).await?;
                stream.flush().await?;
                let _ = client_log(ip, &cpid, GET_HEADER, NOT_FOUND).await?;
                return Ok(NOT_FOUND);
            }
        }
        DELETE => {
            let media = Media {
                name: request.name,
                size: request.size,
                cpid: request.cpid.clone(),
                type_: request.type_,
                checksum: request.chcksum,
                path: String::new(),
                in_host: host_cpid.clone()
            };
            if check_if_media_exist_wchecksum(
                &media,
                pool
            ).await {
                let db_media = Media::get(
                    host_cpid,
                    &media.cpid,
                    &media.checksum,
                    pool
                ).await.unwrap();
                let path = PathBuf::from_str(&db_media.path)
                    .unwrap();
                info!("deleteing: {}", &db_media.path);
                let rows = delete_media(db_media, pool)
                    .await.unwrap();
                assert!(rows == 1);
                remove_file(path).unwrap();
                let _ = client_log(ip, &cpid, DELETE_HEADER, 0).await?;

            } else {
                stream.write_u8(NOT_FOUND).await?;
                stream.flush().await?;
                let _ = client_log(ip, &cpid, DELETE_HEADER, NOT_FOUND).await?;
                return Ok(NOT_FOUND);
            }
        }
        _ => {}
    }
    Ok(0)
}

pub async  fn handle_server_request(
    request: RQM,
    stream: &mut ServerTlsStreams,
    client_ip: IpAddr,
    host_cpid: &String,
    pool: &PgPool
) -> Result<u8> {
    let ip = client_ip;
    let cpid = request.cpid.clone();
    match request.header.as_str() {
        POST => {
            if check_if_media_exist(
                &request.cpid,
                host_cpid,
                &request.name,
                &request.type_,
                request.size,
                pool
            ).await {
                stream.write_u8(MEDIA_ALREADY_EXISTS).await?;
                stream.flush().await?;
                
            } else {
            debug!("SERVER: handling( client post request )");
            let mut path = PathBuf::new();
            path.push(DATA_DIR);
            let name = lib_db::fncs::random_string(16);
            path.push(name);
            let f = File::create_new(&path).await?;
            info!("SERVER: created {:#?} ",&path);
            let spath = path.to_str().unwrap();
            let mut writer = BufWriter::new(f);
            stream.write_u8(READY_STATUS).await?;
            let _size = wifb(
                    Some(stream),
                    None,
                    &mut writer,
                ).await?;
            let local_sum = get_fsum(spath).await?;
            let local_size = get_size(spath).await?;
            if &request.chcksum == NO_VAL {
                debug!("a client sent a file with no checksum");
                wvts(Some(stream), None, local_sum.as_bytes().to_vec())
                        .await
                        .unwrap();
            }else if request.chcksum != local_sum {
                stream.write_u8(DATA_NOT_MATCH).await?;
                stream.flush().await?;
            };
            if request.size != local_size {
                stream.write_u8(DATA_NOT_MATCH).await?;
                stream.flush().await?;
            };
            let media = Media {
                name: request.name,
                cpid: request.cpid,
                path: spath.to_owned(),
                checksum: local_sum,
                in_host: host_cpid.clone(),
                type_: request.type_,
                size: local_size,
            };
            let _res = media.post(pool).await.unwrap();
            assert_eq!(_res, 0);
            let _ = client_log(ip, &cpid, POST_HEADER, 0).await?;
            }
        }
        GET => {
            if check_if_media_exist(
                &request.cpid,
                host_cpid,
                &request.name,
                &request.type_,
                request.size,
                pool
            ).await { 
                let media = Media::get(
                    host_cpid,
                    &request.cpid,
                    &request.chcksum,
                    pool
                ).await.unwrap();
                let path = PathBuf::from(&media.path);
                assert_eq!(true, path.exists());
                let metadata = metadata(&path);
                let f = File::open(&path).await?;
                let size = if metadata.is_ok() {
                    let size = metadata.unwrap().len();
                    assert_eq!(size as i64, request.size);
                    size 
                } else  {
                    error!(
                        "could not open file: {}",
                        media.path
                    );
                    0

                };
                if size == 0 {
                    stream.write_u8(SERVER_SIDE_ERR).await?;
                    stream.flush().await?;
                } else {
                    stream.write_u8(READY_STATUS).await?;
                    stream.flush().await?;
                    let ready = stream.read_u8().await?;
                    if ready == READY_STATUS {
                        let mut reader = BufReader::new(f);
                        wffb(
                            Some(stream),
                            None,
                            size,
                            &mut reader,
                        ).await?;
                        let confirm = stream.read_u8().await?;
                        if confirm == SUCCESFUL {
                            info!("SUCCESFUL:GET");
                        } else {
                            debug!("UNSUCCESFUL:GET");
                            let _ = client_log(ip, &cpid, GET_HEADER, 1).await?;
                            return Ok(1)
                        }
                        let _ = client_log(ip, &cpid, GET_HEADER, 0).await?;
                    }
                }
            } else {
                stream.write_u8(NOT_FOUND).await?;
                stream.flush().await?;
                let _ = client_log(ip, &cpid, GET_HEADER, NOT_FOUND).await?;
                return Ok(NOT_FOUND);
            }
        }
        DELETE => {
            let media = Media {
                name: request.name,
                size: request.size,
                cpid: request.cpid,
                type_: request.type_,
                checksum: request.chcksum,
                path: String::new(),
                in_host: host_cpid.clone()
            };
            if check_if_media_exist_wchecksum(
                &media,
                pool
            ).await && host_cpid != &media.cpid {
                let db_media = Media::get(
                    host_cpid,
                    &media.cpid,
                    &media.checksum,
                    pool
                ).await.unwrap();
                let path = PathBuf::from_str(&db_media.path)
                    .unwrap();
                info!("deleteing: {}", &db_media.path);
                let rows = delete_media(db_media, pool)
                    .await.unwrap();
                assert!(rows == 1);
                remove_file(path).unwrap();
                let _ = client_log(ip, &cpid, DELETE_HEADER, 0).await?;

            } else {
                stream.write_u8(NOT_FOUND).await?;
                stream.flush().await?;
                let _ = client_log(ip, &cpid, DELETE_HEADER, NOT_FOUND).await?;
                return Ok(NOT_FOUND);
            }
        }
        _ => {}
    } 
    Ok(0)
}
