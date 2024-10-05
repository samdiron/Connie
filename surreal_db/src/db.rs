use once_cell::sync::Lazy;
use surrealdb::engine::remote::ws::{Client, Ws, Wss};
use surrealdb::Surreal;

pub static DBASE: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);


#[derive()]
pub struct DB<'a> {
    pub addr: &'a str,
    // pub name_sp: &'a str,
    // pub database: &'a str,
    pub remote: bool,

}

impl <'a>DB<'a> {
    pub async fn connect(self) -> surrealdb::Result<()>{

       if self.addr.contains(":") {
           if self.remote == false {
               DBASE.connect::<Ws>(self.addr).await?;
               Ok(())
           }
           else  {
               DBASE.connect::<Wss>(self.addr).await?;
               Ok(())
           }
       }
        else {
            println!("addr is invalid or null");
            Ok(())
        }
    }
}

