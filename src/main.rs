use std::fs::remove_file;

//NOTE: for this progrm to start you have to write your postgres connection url
//like this postgres://db_user:password_for_the_user@ip:port/database_name
//postgres default port is 5432, and ip by default is localhost
//in the /Connie/etc/db_conn; file

fn main() {
    let lockfile = "/Connie/lockfile";
    //start of the program 
    
    //end of the program
    let _ = remove_file(lockfile).unwrap();
}
