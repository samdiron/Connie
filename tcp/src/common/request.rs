use lib_db::{
    jwt::{create, exp_gen, validate_jwt_claim, Claim}, media::checksum, types::{jwtE, sqlE, PgPool}, user::user_struct::validate_claim
};
use tokio::fs::File;
use std::{os::unix::fs::MetadataExt, path::PathBuf, str::FromStr};
use serde::{Deserialize, Serialize};
use bincode::{self};

pub const READY_STATUS: u8 = 01;

pub const UNAUTHORIZED: u8 = 41;

pub const DATA_NOT_MATCH: u8 = 66;

pub const SUCCESFUL: u8 = 0;

pub const RECONNECT_STATUS: u8 = 8;

pub const JWT_AUTH: u8 = 0;

pub const LOGIN_CRED: u8 = 1;

pub const SIGNIN_CRED: u8 = 2;

pub const PACKET_SIZE: usize = 65533usize;


pub const GET: &str = "!G";

pub const POST: &str = "!P";

pub const DELETE: &str = "!D";





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
    pub async fn create(path: PathBuf, header: String, cpid: String) -> std::io::Result<Self> {
        let f = File::open(path.clone()).await?;
        let data = f.metadata().await?;
        let size = data.size() as i64;

        let str_type = path.extension().unwrap().to_str().unwrap();
        let type_ = String::from_str(str_type).unwrap();

        let str_name = path.file_name().unwrap().to_str().unwrap();
        let name = String::from_str(str_name).unwrap();
        
        let path = path.to_str().unwrap();
        let chcksum = checksum::get_fsum(path).await?;

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

#[derive(Deserialize, Serialize)]
pub struct JwtReq {
    pub jwt: String,
    pub request: RQM
}

#[derive(Deserialize, Serialize)]
pub struct LoginReq {
    pub cpid: String,
    pub name: String,
    pub paswd: String,
}

impl LoginReq {
    pub fn drop(self) {
        drop(self);
    }

    pub async fn validate(self, pool: &PgPool) -> Result<bool, sqlE> {
        let paswd = self.paswd.clone();
        let name = self.name.clone();
        let res = validate_claim(name, paswd, pool).await?;
        Ok(res)

    } 

    pub async fn token_gen(self) -> Result<String, jwtE> {
        let cpid = self.cpid;
        let paswd = self.paswd;

        let exp = exp_gen();
        let claim = Claim {
            cpid,
            paswd,
            exp
        };

        let token = create(&claim).await?;
        Ok(token)
        
       
    }

    pub fn sz(self) -> Result<Vec<u8>, bincode::Error> {
        let res: Vec<u8> = bincode::serialize(&self)?;
        Ok(res)
    }
    
    pub fn dz(m: Vec<u8>) -> Result<Self, bincode::Error> {
        let res: Self = bincode::deserialize(&m)?;
        Ok(res) 
    }

}

impl JwtReq {
    
    pub fn drop(self) {
        drop(self);
    }

    pub async fn validate(&mut self, pool: &PgPool) -> Result<bool, sqlE> {
        let token = &self.jwt;
        let state = validate_jwt_claim(token, pool).await;
        Ok(state)
    }

    pub fn sz(self) -> Result<Vec<u8>, bincode::Error> {
        let res: Vec<u8> = bincode::serialize(&self)?;
        Ok(res)
    }

    pub fn dz(m: Vec<u8>) -> Result<Self, bincode::Error> {
        let res: Self = bincode::deserialize(&m)?;
        Ok(res) 
    }

} 







