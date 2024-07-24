//use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tokio;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db: Surreal<Client> = Surreal::new::<Ws>("0.0.0.0:8000").await?;
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    db.use_ns("test").use_db("test2").await?;

    println!("done");
    Ok(())
}
