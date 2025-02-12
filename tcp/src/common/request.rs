use lib_db::{
    jwt::{self, create, exp_gen, validate_jwt_claim, Claim}, media::checksum, types::{jwtE, sqlE, PgPool}, user::user_struct::validate_claim
};
use tokio::{fs::File, io::AsyncWriteExt, net::TcpStream};
use std::{fs::FileType, io::{Error, ErrorKind}, mem::ManuallyDrop, os::unix::fs::MetadataExt, path::PathBuf, str::FromStr};
use serde::{Deserialize, Serialize};
use bincode::{self};



#[allow(dead_code)]
pub const JWT_AUTH: u8 = 0;

#[allow(dead_code)]
pub const LOGIN_CRED: u8 = 1;

#[allow(dead_code)]
pub const SIGNIN_CRED: u8 = 2;

#[allow(dead_code)]
pub const PACKET_SIZE: u16 = 65535;


#[allow(dead_code)]
pub const GET: &str = "!G";

#[allow(dead_code)]
pub const POST: &str = "!P";

#[allow(dead_code)]
pub const DELETE: &str = "!D";





#[derive(Deserialize, Serialize)]
pub struct RQM {
    size: i64,
    name: String,
    type_: String,
    chcksum: String,
    in_host: String,
}

impl RQM {
    pub async fn create(path: PathBuf, in_host: String) -> std::io::Result<Self> {
        let f = File::open(path.clone()).await?;
        let data = f.metadata().await?;
        let size = data.size() as i64;

        let str_type = path.extension().unwrap().to_str().unwrap();
        let type_ = String::from_str(str_type).unwrap();

        let str_name = path.file_name().unwrap().to_str().unwrap();
        let name = String::from_str(str_name).unwrap();
        
        let _path = path.to_str().unwrap();
        let chcksum = checksum::get(_path).await?;

        Ok(RQM {
            size,
            name,
            type_,
            chcksum,
            in_host
        })
    }
}

#[derive(Deserialize, Serialize)]
pub struct JwtReq {
    pub jwt: String,
    pub request_type: u8,
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







