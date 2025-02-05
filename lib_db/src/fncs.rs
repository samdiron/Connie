use std::str::FromStr;
use uuid::Error;

pub async fn valdiate_uuid(uuid: &str, v: u8)  -> Result<bool,  Error> {
    let uuid = uuid::Uuid::from_str(uuid)?;
    let version = uuid.get_version_num() as u8;
    if version == v {
        Ok( true)
    }else {
        Ok( false)
    }
}
