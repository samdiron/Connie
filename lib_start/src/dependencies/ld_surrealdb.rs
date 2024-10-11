use std::fmt::format;
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
        let path = format!("{}/surrealLogs/logs.csv",home_path) ;
        let mut  file = File::open(path).expect("could not open logs.csv");
        file.write_all(error_data).expect("could not write to logs.csv");
        return 1
    }

}
pub fn strat_command(full_ip:&str) {
    let command_string = format!("surreal start --web-crt '~/Connie/cert/cert.pem' --web-key '~/Connie/key/key.pem' -b '{i}'",i = full_ip);
    let surreal_command  = Command::new("sh").arg("surreal")
        .arg("start")
        .arg("--web-crt")
        .arg("'~/Connie/cert/cert.pem'")
        .arg("--web-key")
        .arg("'~/Connie/key/key.pem'")
        .arg("-b")
        .arg(full_ip.as_str())
        .output().expect("could not run surreal db start command");

}