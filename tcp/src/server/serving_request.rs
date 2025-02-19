use std::io::Result;
use std::path::PathBuf;
use common_lib::cheat_sheet::gethostname;
use lib_db::media::checksum::{get_fsum, get_size};
use lib_db::media::media::Media;
use lib_db::types::PgPool;
use log::debug;
use tokio::io::AsyncWriteExt;
use tokio::{io::BufWriter, net::TcpStream};
use tokio::fs::File;
use crate::common::request::{DATA_NOT_MATCH, GET, READY_STATUS};
use crate::common::util::wifb;
use crate::types::{POST, RQM};
use common_lib::path::DATA_DIR;



pub async  fn handle_server_request(
    request: RQM,
    stream: &mut TcpStream,
    pool: &PgPool
) -> Result<u8> {
    match request.header.as_str() {
        POST => {
            println!("SERVER: handling( client post request )");
            debug!("SERVER: handling( client post request )");
            let mut path = PathBuf::new();
            path.push(DATA_DIR);
            let name = lib_db::fncs::random_string(8);
            path.push(name);
            let f = File::create_new(&path).await?;
            println!("SERVER: created {:#?} ",&path);
            let spath = path.to_str().unwrap();
            let mut writer = BufWriter::new(f);
            println!("SERVER: wifb");
            stream.write_u8(READY_STATUS).await?;
            let _size = wifb(stream, &mut writer).await?;
            let local_sum = get_fsum(spath).await?;
            let local_size = get_size(spath).await?;
            if (request.size != local_size) || (request.chcksum != local_sum) {
                stream.write_u8(DATA_NOT_MATCH).await?;
                stream.flush().await?;
            };
            stream.write_u8(0).await?;
            let media = Media {
                name: request.name,
                cpid: request.cpid,
                path: spath.to_owned(),
                checksum: local_sum,
                in_host: gethostname().to_str().unwrap().to_owned(),
                type_: request.type_,
                size: local_size,
            };
            let _res = media.post(pool).await.unwrap();
            assert_eq!(_res, 0);

        }
        GET => {}
        _ => {}
    } 
    Ok(0)
}
