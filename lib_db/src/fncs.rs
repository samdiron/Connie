use std::str::FromStr;
use uuid::Error;
use common_lib::rand::{self, distributions::Alphanumeric, Rng}; 





pub fn escape_user_input(s: &String) -> String{
    let bind = s.replace("'", "");
    bind
}

pub fn random_string(chars: u8) -> String{
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(chars as usize)
        .map(char::from)
        .collect();
    s
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
