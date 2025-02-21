use std::str::FromStr;
use uuid::Error;
use rand::random;

pub fn random_string(chars: u8) -> String {
    (0..chars).map(|_| (0x20u8 + (random::<f32>() * 96.0) as u8) as char).collect()
}

pub async fn valdiate_uuid(uuid: &str, v: u8)  -> Result<bool,  Error> {
    let uuid = uuid::Uuid::from_str(uuid)?;
    let version = uuid.get_version_num() as u8;
    if version == v {
        Ok( true)
    }else {
        Ok( false)
    }
}
