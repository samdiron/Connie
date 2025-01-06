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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use database::POOL;
//     use sqlx::{self, Error, PgPool, Result};
//     use std::time::Duration;
//     use std::time::Instant;
//     #[tokio::test]
//     async fn db_conn_speed() {
//
//     }
// }
