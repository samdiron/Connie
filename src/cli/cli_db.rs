
use std::{
    fs::remove_file,
    process::exit
};

use lib_db::{database::get_conn, sqlite};

use tokio::{fs::File, io::AsyncWriteExt};

use common_lib::path::{DB_CONN, SQLITEDB_PATH};

use crate::Commands;


pub async fn handle_cli_db(command: Commands) {
    match command {
         Commands::DB { 
            migrations,
            connection,
            delete_conn,
            test
        } => {
            if let Some(conn) = connection {
                let mut f = File::create_new(DB_CONN)
                    .await
                    .expect("this command creates a new file in /opt/Connie/conf it needs to be executed by root ");
                f.write_all(conn.as_bytes())
                    .await
                    .unwrap();
            }
            if let Some(delete_conn) = delete_conn {
                if delete_conn == true {
                    remove_file(DB_CONN).expect("this command deletes a file this will need root");
                    println!("deleted {DB_CONN} file");
                    exit(0)
                }
            }
            if let Some(migrations) = migrations {
                let pool =  get_conn().await.unwrap();
                let pool = &pool;
                let spool = sqlite::get_sqlite_conn(
                    &SQLITEDB_PATH.to_owned()
                ).await.unwrap();
                if migrations == true {
                    lib_db::database::migrate(pool).await.unwrap();
                    lib_db::sqlite::migration(&spool).await.unwrap();
                }
            }
            if let Some(test) = test {
                if test == true {
                    let _pool = get_conn()
                        .await
                        .expect("can't connect to db");
                    println!("db connection valid");
                    exit(0)
                }
            } 
        }
        _=> {}

    }
}
