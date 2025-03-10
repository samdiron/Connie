use lib_db::media::checksum;

use std::{
    fs::metadata,
    path::PathBuf, str::FromStr
};
use serde::{Deserialize, Serialize};

pub const READY_STATUS: u8 = 01;

pub const UNAUTHORIZED: u8 = 41;

pub const DATA_NOT_MATCH: u8 = 66;

pub const SUCCESFUL: u8 = 0;

pub const RECONNECT_STATUS: u8 = 8;

pub const JWT_AUTH: u8 = 0;

pub const NOT_FOUND: u8 = 22;
pub const CLIENT_SIDE_ERR: u8 = 68;
pub const SERVER_SIDE_ERR: u8 = 69;

pub const MEDIA_ALREADY_EXISTS: u8 = 62;

pub const LOGIN_CRED: u8 = 1;

pub const SIGNIN_CRED: u8 = 2;

pub const PACKET_SIZE: usize = 65533usize;

pub const FETCH: u8 = 14;

pub const GET: &str = "!G";

pub const POST: &str = "!P";

pub const DELETE: &str = "!D";

pub(crate) use crate::server::req_format;

pub(crate) const NO_VAL: &str = "N//A";

#[derive(Deserialize, Serialize)]
#[derive(Clone)]
pub struct RQM {
    pub size: i64,
    pub cpid: String,
    pub name: String,
    pub type_: String,
    pub header: String,
    pub chcksum: String,
    pub path: Option<String>
}


impl RQM {
    pub async fn create(path: PathBuf, header: String, cpid: String, create_checksum: bool) -> std::io::Result<Self> {
        let data = metadata(&path)?;
        let size = data.len() as i64;

        let ext = path.extension();
        let type_ = if ext.is_some() {
            let bind = String::from_str(
                ext.unwrap()
                    .to_str()
                    .unwrap()
            ).unwrap();
            bind
        }else {
            let bind = String::new(); 
            bind
        };

        let str_name = path.file_name().unwrap().to_str().unwrap();
        let name = String::from_str(str_name).unwrap();
        
        let path = path.to_str().unwrap();
        let chcksum = if create_checksum {
            checksum::get_fsum(path).await?
        }else {NO_VAL.to_string()};

        Ok(RQM {
            size,
            name,
            cpid,
            type_,
            header,
            chcksum,
            path: Some(path.to_owned())
        })
    }
}






