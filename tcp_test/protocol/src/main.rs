use surrealdb::Surreal;
use surrealdb::sql::Thing;
use surrealdb::opt::auth::Root;
use surrealdb::engine::remote::ws::Ws;
use tokio;
use bcrypt::{hash, DEFAULT_COST};
use srade_json::json;


#[tokio::main]
async fn main() -> Result<(),Box<dny std::error::Error>> {
    //connect to db
    let db = Surreal::new::<Ws>(0.0.0.0:8000).await?;

    //sgin in
    db.sginin(Root{
        username: "root",
        password: "root",
    }).await?;
    db.
}
