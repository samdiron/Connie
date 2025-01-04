#[allow(clippy::upper_case_acronyms)]
pub mod database;
pub mod media;
mod migrations;
pub mod server;
pub mod user;

pub mod JWT {

    pub async fn create() {}
    pub async fn validate() {}
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
