use serde::{Deserialize, Serialize};
use std::env;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tokio;

#[derive(Debug, Serialize)]
struct Name<'a> {
    first: &'a str,
    last: &'a str,
}

#[derive(Debug, Serialize)]
struct Person<'a> {
    title: &'a str,
    name: Name<'a>,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("not all args");
    }
    let fname: &String = &args[1];
    let lname: &String = &args[2];
    let user_s: &String = &args[3];

    let db: Surreal<Client> = Surreal::new::<Ws>("0.0.0.0:8000").await?;
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    db.use_ns("test").use_db("test2").await?;
    db.set(
        "name",
        Name {
            first: fname,
            last: lname,
        },
    )
    .await?;

    db.set("job", user_s.to_string()).await?;
    db.query("CREAT person SET name=$name , hashed=crypt::sha256($job")
        .await?;
    println!("done");
    Ok(())
}