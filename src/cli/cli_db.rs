
use std::{
    fs::remove_file, process::exit
};

use lib_db::{database::{get_conn, migrate}, sqlite::{self, migration}};

use tokio::{fs::File, io::AsyncWriteExt};

use common_lib::path::{DB_CONN, SQLITEDB_PATH};

use crate::Commands;


pub async fn handle_cli_db(command: Commands) {
    match command {
         Commands::DB { 
            test,
            path,
            connection,
            delete_conn,
            sqlite_migrations,
            postgres_migrations,
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
            if sqlite_migrations.is_some() && sqlite_migrations.unwrap() {
                let path = if path.is_some() {
                    path.unwrap().to_str().unwrap().to_string()
                } else {SQLITEDB_PATH.to_string()};
                let spool = sqlite::get_sqlite_conn(
                    &path
                ).await.unwrap();
                migration(&spool).await.unwrap()
            }
            if postgres_migrations.is_some() && postgres_migrations.unwrap() {
                let conn = get_conn().await.unwrap();
                migrate(&conn).await.unwrap();
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
