use std::fs::File;
use std::process::Command;
use std::io::{Error, ErrorKind, Write};

pub fn check(home_path : &str) -> u8 {
    let surreal_db_check = Command::new("sh")
        .arg("surreal")
        .arg("--version")
        .output().is_ok();
        //.expect("surrealdb check failed to start");
    if surreal_db_check == true {
        return 0
    }
    else {
        let error_data = format!("{},\n",(Error::new(ErrorKind::NotFound , "SurrealDB not found"))).as_bytes();
        println!("{}",error_data);
        let mut  file = File::open("{}/surrealLogs/logs.csv",home_path).expect("could not open logs.csv");
        file.write_all(error_data).expect("could not write to logs.csv");
        return 1
    }

}
