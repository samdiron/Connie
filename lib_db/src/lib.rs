#[allow(clippy::upper_case_acronyms)]
pub mod database;
pub mod media;
pub mod fncs;
mod migrations;
pub mod server;
pub mod user;

#[allow(unused_imports)]
pub(crate) use common_lib::sha256;
use common_lib::sha256::digest;

pub fn escape_user_input(s: &String) -> String{
    // removing sigle quotes
    let ns = s.replace("'", "");
    // removing double quotes
    let ns = ns.replace(r#"""#, "");
    ns
}


pub fn hash_passwords(s: String) -> String {
    let hashed = digest(s);
    hashed
}

pub mod inner {
    pub use sqlx;
    pub use crate::types;
    pub use crate::sqlite;
    pub use crate::hash_passwords;
    pub use crate::escape_user_input;
}

/// this lib provides a sqlite db for the client to make it easer for 
/// non technical users
pub mod sqlite;

/// provides useful types for functions
pub mod types {
    pub use crate::jwt::Claim;
    pub use jsonwebtoken::errors::Error as jwtE;
    pub use jsonwebtoken::errors::Result;
    pub use sqlx::Error as sqlE;
    pub use sqlx::{PgPool, Postgres, SqlitePool};
}

pub mod jwt {
    
    pub const DURATION: u64 = 86400;

    use std::sync::{LazyLock, Mutex};

    use jsonwebtoken::{
        decode,
        encode,
        Header,
        Validation,
        DecodingKey,
        EncodingKey,
        errors::Result,
    };
    pub use jsonwebtoken::get_current_timestamp;

    pub fn exp_gen() -> u64 {
        let now = get_current_timestamp();
        let exp = now + DURATION;
        exp
    }
    use sqlx::PgPool;
    use crate::user::validation::validate_claim_wcpid;
    use serde::{Deserialize, Serialize};
    // the user may chose the word 
    pub static MUTEX_SECRET_WORD: Mutex<LazyLock<Mutex<String>>> = Mutex::new(
            LazyLock::new( ||  
                {
                    Mutex::new(String::from("Lorem ipsum dolor sit amet quis"))
                }
            )
    );

    fn get_secret() -> String {
        let mutex_word = MUTEX_SECRET_WORD
            .lock().unwrap()
            .lock().unwrap()
            .clone();
        let str = mutex_word;
        str
    }

    pub const SECRET_WORD: LazyLock<String> = LazyLock::new(
        || {get_secret()}
    );


    #[derive(Serialize, Deserialize, Debug)]
    pub struct Claim {
        pub cpid: String,
        pub paswd: String,
        pub exp: u64,
    }
    pub async fn create(claim: &Claim) -> Result<String> {
        let token = encode(
            &Header::default(),
            &claim,
            &EncodingKey::from_secret(SECRET_WORD.as_ref()),
        )?;
        Ok(token)
    }
    fn decode_jwt(token: &String) -> Result<Claim> {
        let token = decode::<Claim>(
            token,
            &DecodingKey::from_secret(SECRET_WORD.as_ref()),
            &Validation::default(),
        )?;

        Ok(token.claims)
    }
    /// takes the raw jwt then decodes it and validate it with the server cpid
    pub async fn validate_jwt_claim(
        token: &String,
        cpid: &String,
        pool: &PgPool
    ) -> (bool, String) {
        let c = decode_jwt(token);
        let mut client_cpid = String::new();
        if !c.is_ok(){
            return (false, client_cpid)
        }
        let c = c.unwrap();
        let now = get_current_timestamp();
        let is_who = validate_claim_wcpid(
            &c.cpid,
            &c.paswd,
            cpid,
            pool
        ).await.unwrap();
        let exp = c.exp;
        if is_who && (now < exp) {
            client_cpid = c.cpid.clone();
            return (true, client_cpid )
        }else {
            return (false, client_cpid)
        }
        
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    use sqlx::{self, query, Row};
    use std::time;
    #[test]
    fn db_conn_speed() {
        common_lib::tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let key = "POSTGRES_CLUSTER";
                match std::env::var(key) {
                    Ok(ky) => {
                        let sql = "SELECT 4 + 6 AS SUM";
                        let t1 = time::Instant::now();
                        let _pool = sqlx::postgres::PgPool::connect(ky.as_str()).await.unwrap();
                        let res = query(sql).fetch_one(&_pool).await.unwrap();
                        let sum: i32 = res.get("SUM");
                        assert_eq!(sum, 10);
                        let t2 = time::Instant::now();
                        let time = t2 - t1;
                        println!("duration: {}µ", time.as_micros());
                    }
                    Err(e) => {
                        println!("{}", e)
                    }
                }
            });
    }
}
