use crate::db::DB;
use serde::{Deserialize, Serialize};
// use surrealdb::opt::auth::Record;
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize)]
pub struct LocalMachine {
    pub cpid: Uuid,
    pub passwd: String,
    pub host_name: String,
    pub hardware: Hardware,
    pub status: u8,
    pub server_name: String
    // pub max_client: u32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Hardware {
    pub cpu_core_count: usize,
    pub swap: u64,
    pub memory: u64,
}
pub async fn start_minfo() -> surrealdb::Result<LocalMachine> {
    let db = DB.clone();
    db.use_ns("local_unit").use_db("private_infer").await?;
    let machine: Option<LocalMachine> = db.select(("unit", "localmachine")).await?;
    let unwrapped = machine.unwrap();
    let machine = unwrapped;
    Ok(machine)
}

impl LocalMachine {
    pub async fn create(self) -> surrealdb::Result<()> {
        let db = DB.clone();
        db.use_ns("local_unit").use_db("private_infer").await?;
        let created: Option<LocalMachine> = db
            .create(("unit", "local_machine"))
            .content(LocalMachine {
                host_name: self.host_name,
                cpid: self.cpid,
                passwd: self.passwd,
                status: self.status,
                server_name: self.server_name,
                // max_client: self.max_client,
                hardware: Hardware {
                    cpu_core_count: self.hardware.cpu_core_count,
                    swap: self.hardware.swap,
                    memory: self.hardware.memory,
                },
            })
            .await.expect("could not create machine");
        dbg!(created);
        Ok(())
    }
}
