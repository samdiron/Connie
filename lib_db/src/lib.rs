#[allow(clippy::upper_case_acronyms)]
pub mod database;
pub mod media;
mod migrations;
pub mod server;
pub mod user;

pub mod types {
    pub use crate::jwt::Claim;
    pub use jsonwebtoken::errors::Error as jwtE;
    pub use jsonwebtoken::errors::Result;
    pub use sqlx::Error as sqlE;
    pub use sqlx::{PgPool, Postgres};
}

pub mod jwt {

    use jsonwebtoken::{
        decode, encode, errors::Result, DecodingKey, EncodingKey, Header, Validation,
    };
    use serde::{Deserialize, Serialize};

    static SECRET_WORD: &str = r#"word"#;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Claim {
        pub cpid: String,
        pub paswd: String,
        pub exp: usize,
    }
    pub async fn create(clam: &Claim) -> Result<String> {
        let token = encode(
            &Header::default(),
            &clam,
            &EncodingKey::from_secret(SECRET_WORD.as_ref()),
        )?;
        Ok(token)
    }
    pub async fn validate(token: &String) -> Result<Claim> {
        let token = decode::<Claim>(
            token,
            &DecodingKey::from_secret(SECRET_WORD.as_ref()),
            &Validation::default(),
        )?;

        Ok(token.claims)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    use sqlx::{self, query, Row};
    use std::time;
    #[test]
    fn db_conn_speed() {
        tokio::runtime::Builder::new_multi_thread()
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
