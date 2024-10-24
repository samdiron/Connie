use std::fs::exists;
use std::fs::{create_dir_all, File};
use std::io::{Error, ErrorKind, Write};
use std::process::Command;
use crate::common::path::h_path;


pub fn surreal_ld_check(home_path: &str) -> u8 {
    let surreal_db_check = Command::new("sh")
        .arg("surreal")
        .arg("--version")
        .output()
        .is_ok();
    //.expect("surreal check failed to start");
    return if surreal_db_check == true {
        0
    } else {
        let error_data = format!(
            "{},\n",
            Error::new(ErrorKind::NotFound, "SurrealDB not found")
        );
        println!("{}", error_data);
        let path = format!("{}/surreal/logs.csv", home_path);
        let check = exists(path.as_str()).unwrap();
        if check == false {
            //let path_to_l = format!("{}/surreal",home_path);
            File::create_new(path.as_str())
                .expect("could not make a surreal/logs.csv");
        };
        let mut file = File::open(path).expect("could not open logs.csv");
        file.write_all(error_data.as_bytes())
            .expect("could not write to logs.csv");
        1
    }
}
pub fn start_db_command(ip: &str) -> i32 {
    println!("process: starting SurrealDB");
    let hp = h_path();
    let hp2 = hp.clone();
    let hp3 = hp.clone();
    let db_path = format!("rocksdb:{hp}/Connie/surreal/Connie.db");
    let cert_p = format!("{hp2}/Connie/cert/cert.pem");
    let key_p = format!("{hp3}/Connie/cert/cert.pem");
    let full_ip = format!("{}:8060", ip);
    Command::new("sh")
        .arg("surreal")
        .arg("start")
        .arg(db_path)//"rocksdb:/Connie/surreal/Connie.db")
        .arg("--web-crt")
        .arg(cert_p)
        .arg("--web-key")
        .arg(key_p)
        .arg("-b")
        .arg(full_ip.as_str())
        .output()
        .expect("could not run surreal db start command");
    println!("finished: SurrealDB started on port 8060");
    return 0;
}
