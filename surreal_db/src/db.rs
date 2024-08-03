use once_cell::sync::Lazy;
use surrealdb::engine::remote::ws::{Client, Ws, Wss};
use surrealdb::Surreal;

static DBASE: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);


#[derive()]
pub struct DB {
    pub addr: Option<String>,
    pub name_sp: Option<String>,
    pub database: Option<String>,
    pub remote: bool
}

impl DB {
    pub async fn connect(self) -> surrealdb::Result<()>{
       if self.addr.contains(":") {
           if self.remote == false {
               DBASE.connect::<Ws>(self.addr).await?;
           }
           else  {
               DBASE.connect::<Wss>(self.addr).await?;
           }
       }
        else { println!("addr is invalid or null"); }
    }
}

