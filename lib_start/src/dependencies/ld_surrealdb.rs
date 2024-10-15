use std::fmt::format;
#[allow(special_module_name)]
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
        let error_data = format!("{},\n",Error::new(ErrorKind::NotFound , "SurrealDB not found"));
        println!("{}",error_data);
        let path = format!("{}/surrealLogs/logs.csv",home_path) ;
        let mut  file = File::open(path).expect("could not open logs.csv");
        file.write_all(error_data.as_bytes()).expect("could not write to logs.csv");
        return 1
    }

}
pub fn start_db_command(ip: &str) -> u8 {
    let full_ip = format!("{}:8060",ip).as_str();
    let surreal_command  = Command::new("sh").arg("surreal")
        .arg("start")
        .arg("--web-crt")
        .arg("'~/Connie/cert/cert.pem'")
        .arg("--web-key")
        .arg("'~/Connie/key/key.pem'")
        .arg("-b")
        .arg(full_ip)
        .output().expect("could not run surreal db start command");
    return 0

}