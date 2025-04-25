
use std::io::Result;
use std::fs::metadata;
use std::path::PathBuf;

use lib_db::types::PgPool;
use lib_db::media::checksum::{get_fsum, get_size};
use lib_db::media::media::{
    check_if_media_exist, check_if_media_exist_wchecksum, delete_media, Media
};

use common_lib::path::DATA_DIR;
use common_lib::tokio::fs::File;
use common_lib::tokio::io::BufWriter;
use common_lib::tokio::io::AsyncWriteExt;
use common_lib::log::{debug, error, info};
use common_lib::tokio::io::{AsyncReadExt, BufReader};

use crate::types::RQM;
use crate::common::ServerTlsStreams;
use crate::common::util::server::{wffb, wifb, wvts};
use crate::common::request::{
    DATA_NOT_MATCH,
    GET,
    DELETE,
    POST,
    MEDIA_ALREADY_EXISTS,
    NOT_FOUND,
    NO_VAL,
    READY_STATUS,
    SERVER_SIDE_ERR,
    SUCCESFUL
};



pub async  fn handle_server_request(
    request: RQM,
    stream: &mut ServerTlsStreams,
    host_cpid: &String,
    pool: &PgPool
) -> Result<u8> {
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
            let _size = wifb(stream, &mut writer, false).await?;
            let local_sum = get_fsum(spath).await?;
            let local_size = get_size(spath).await?;
            if &request.chcksum == NO_VAL {
                debug!("a client sent a file with no checksum");
                wvts(stream, local_sum.as_bytes().to_vec())
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
                    error!("could not open file: {}",media.path);
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
                        wffb(stream, size, &mut reader, false).await?;
                        let confirm = stream.read_u8().await?;
                        if confirm == SUCCESFUL {
                            info!("SUCCESFUL:GET");
                        } else {
                            debug!("UNSUCCESFUL:GET");
                            return Ok(1)
                        }
                    }
                }
            } else {
                stream.write_u8(NOT_FOUND).await?;
                stream.flush().await?;
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
            ).await {
                let rows = delete_media(media, pool).await.unwrap();
                assert!(rows == 1);

            } else {
                stream.write_u8(NOT_FOUND).await?;
                stream.flush().await?;
                return Ok(NOT_FOUND);
            }
        }
        _ => {}
    } 
    Ok(0)
}
