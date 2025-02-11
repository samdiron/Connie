use serde::{Deserialize, Serialize};
use std::result::Result;

#[derive(Deserialize, Serialize)]
pub struct RQM {
    request: String,
    size: i64,
    name: String,
    type_: String,
    chcksum: String,
    in_host: String,
}


impl RQM {
    pub fn serialize(self) -> Result<Vec<u8>, bincode::Error> {
        let res: Vec<u8> = bincode::serialize(&self)?;
        Ok(res)
    }
    pub fn desiralize(m: Vec<u8>) -> Result<Self, bincode::Error> {
        let res: Self = bincode::deserialize(&m)?;
        Ok(res) 
    }
}
