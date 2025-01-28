// use std::fs::remove_file;
use lib_db::database::get_conn;
use tcp::server::listener;


//NOTE: for this progrm to start you have to write your postgres connection url
//like this postgres://db_user:password_for_the_user@ip:port/database_name
//postgres default port is 5432, and ip by default is localhost
//in the /Connie/etc/db_conn; file


#[tokio::main]
async fn main() {
    //start of the program 
    let pool =  get_conn().await.unwrap();
    listener::bind(pool).await;
    //end of the program
}
