use once_cell::sync::Lazy;
use surrealdb::engine::remote::ws::{Client, Ws, Wss};
use surrealdb::Surreal;

pub static DBASE: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

#[derive()]
pub struct DB<'a> {
    pub addr: &'a str,
    pub remote: bool,
}

impl<'a> DB<'a> {
    pub async fn connect(self) -> surrealdb::Result<()> {
        let ip = format!("{}:8060", self.addr);
        if !self.remote {
            DBASE.connect::<Ws>(ip.as_str()).await?;
            Ok(())
        } else {
            DBASE.connect::<Wss>(ip.as_str()).await?;
            Ok(())
        }
    }
}
