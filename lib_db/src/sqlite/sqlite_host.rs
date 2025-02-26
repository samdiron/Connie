use sqlx::{Connection, SqlitePool};

pub async fn create_table() {
    let conn = SqlitePool::connect("/opt/Connie/bin/.database").await.unwrap();
    

}
