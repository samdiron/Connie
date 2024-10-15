use std::fs::File;
use std::io::{Error, ErrorKind};
use std::process::exit;
//use num_cpus;
use sysinfo::{Disks, System};
use crate::dependencies::ld_surrealdb::start_db_command;
use crate::first::new::first_time;
use local_ip_address::local_ip;
use surreal_db::db::DB;
pub fn start() {
    let os = System::name();
    if os.unwrap().as_str() == "Microsoft Windows" {
        println!("you are on Microsoft Windows she don't like that");
        Error::new(ErrorKind::Unsupported, "no Microsoft Windows support");
        exit(13); // it means the os is window and they out of luck
    };

    let mut connie_config = File::open("/.config/connie/connie_config.yaml");
    if connie_config.is_ok() {
        let first_time = false;
        let ip = format!("{}",local_ip().expect("Error can't get ip addr")).as_str();
        let start_db = start_db_command(ip);
        DB{
            addr: ip,
            remote: false
        };



        // let dependency_check = check_dependencies().is_ok();
        // if dependency_check == false {
        //     exit(2) //unmet dependency
        // }
    } else {
        first_time()
    };
}