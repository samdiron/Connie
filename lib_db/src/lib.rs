#[allow(clippy::upper_case_acronyms)]
pub mod database;
pub mod media;
mod migrations;
pub mod server;
pub mod user;

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use database::POOL;
//     use sqlx::{self, Error, PgPool, Result};
//     use tokio::runtime;
// }
