
use lib_db::jwt::{
    Claim,
    create,
    exp_gen,
    validate_jwt_claim,
};

use lib_db::{
    types::{jwtE, sqlE, PgPool},
    user::validation::validate_claim_wcpid
};

use common_lib::bincode;
use common_lib::serde::{Deserialize, Serialize};

use crate::common::request::RQM;


#[derive(Deserialize, Serialize)]
pub struct Chead {
    pub jwt: String,
    pub cpid: String,
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

    pub(in crate::server) async fn validate(
        &self,
        host_cpid: &String,
        pool: &PgPool
    ) -> Result<bool, sqlE> {
        let paswd = &self.paswd;
        let cpid = &self.cpid;
        let res = validate_claim_wcpid(cpid, paswd, host_cpid, pool).await?;
        Ok(res)

    } 

    pub(in crate::server)  async fn token_gen(
        self
    ) -> Result<String, jwtE> {
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
        self.drop();
        Ok(res)
    }
    
    pub fn dz(buf: Vec<u8>) -> Result<Self, bincode::Error> {
        let res: Self = bincode::deserialize(&buf)?;
        drop(buf);
        Ok(res) 
    }

}

impl JwtReq {
    
    pub fn drop(self) {
        drop(self);
    }

    pub(in crate::server) async fn validate(
        &self,
        host_cpid: &String,
        pool: &PgPool
    ) -> Result<bool, sqlE> {
        let token = &self.jwt;
        let state = validate_jwt_claim(token, host_cpid, pool).await;
        Ok(state)
    }

    pub fn sz(self) -> Result<Vec<u8>, bincode::Error> {
        let res: Vec<u8> = bincode::serialize(&self)?;
        self.drop();
        Ok(res)
    }

    pub fn dz(buf: Vec<u8>) -> Result<Self, bincode::Error> {
        let res: Self = bincode::deserialize(&buf)?;
        drop(buf);
        Ok(res) 
    }

} 

impl Chead {
    
    pub fn drop(self) {
        drop(self);
    }
    
    pub(in crate::server) async fn validate(
        &self,
        host_cpid: &String,
        pool: &PgPool
    ) -> Result<bool, sqlE> {
        let token = &self.jwt;
        let state = validate_jwt_claim(token, host_cpid, pool).await;
        Ok(state)
    }

    pub fn sz(self) -> Result<Vec<u8>, bincode::Error> {
        let res: Vec<u8> = bincode::serialize(&self)?;
        self.drop();
        Ok(res)
    }

    pub fn dz(buf: Vec<u8>) -> Result<Self, bincode::Error> {
        let res: Self = bincode::deserialize(&buf)?;
        drop(buf);
        Ok(res) 
    }
}
