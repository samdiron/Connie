use crate::db::DB;
use serde::{Deserialize, Serialize};
use surrealdb::opt::auth::Scope;
use uuid::Uuid;


#[derive(Serialize,Debug, Deserialize)]
pub struct DUser {
    pub is_admin: bool,
    pub user_name: String,
    pub name: String,
    pub cpid: Uuid,
    pub pass: String,
}

