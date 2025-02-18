use crate::common::request::RQM;
use lib_db::{jwt::{create, exp_gen, validate_jwt_claim, Claim}, types::{jwtE, sqlE, PgPool}, user::user_struct::validate_claim_wcpid};
use serde::{Deserialize, Serialize};
use bincode;


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

    pub async fn validate(&self, pool: &PgPool) -> Result<bool, sqlE> {
        let paswd = self.paswd.clone();
        let cpid = self.cpid.clone();
        let res = validate_claim_wcpid(cpid, paswd, pool).await?;
        Ok(res)

    } 

    pub(in crate::server)  async fn token_gen(self) -> Result<String, jwtE> {
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
    
    pub fn dz(m: Vec<u8>) -> Result<Self, bincode::Error> {
        let res: Self = bincode::deserialize(&m)?;
        Ok(res) 
    }

}

impl JwtReq {
    
    pub fn drop(self) {
        drop(self);
    }

    pub(in crate::server) async fn validate(&self, pool: &PgPool) -> Result<bool, sqlE> {
        let token = &self.jwt;
        let state = validate_jwt_claim(token, pool).await;
        Ok(state)
    }

    pub fn sz(self) -> Result<Vec<u8>, bincode::Error> {
        let res: Vec<u8> = bincode::serialize(&self)?;
        self.drop();
        Ok(res)
    }

    pub fn dz(m: Vec<u8>) -> Result<Self, bincode::Error> {
        let res: Self = bincode::deserialize(&m)?;
        Ok(res) 
    }

} 

