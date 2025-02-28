#[allow(clippy::upper_case_acronyms)]
pub mod database;
pub mod media;
pub mod fncs;
mod migrations;
pub mod server;
pub mod user;


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
        decode, encode, errors::Result, get_current_timestamp, DecodingKey, EncodingKey, Header, Validation
    };
    pub fn exp_gen() -> u64 {
        let now = get_current_timestamp();
        let exp = now + DURATION;
        exp
    }
    use sqlx::PgPool;
    use crate::user::user_struct::validate_claim_wcpid;
    use serde::{Deserialize, Serialize};
    // the user may chose the word 
    pub static MUTEX_SECRET_WORD: Mutex<&str> = Mutex::new("Lorem ipsum dolor sit amet quis");

    fn get_secret() -> String {
        let mutex_word = *MUTEX_SECRET_WORD.lock().unwrap();
        let str = mutex_word.to_string();
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
    pub async fn validate_jwt_claim(token: &String, pool: &PgPool) -> bool {
        let c = decode_jwt(token).unwrap();
        let now = get_current_timestamp();
        let is_who = validate_claim_wcpid(c.cpid, c.paswd, pool).await.unwrap();
        let exp = c.exp;
        if is_who && (now < exp) {
            return true
        }else {
            return false
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
