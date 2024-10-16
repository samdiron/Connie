use std::fs::File;
use std::io::{Error, ErrorKind};
use std::process::exit;
//use num_cpus;
use crate::dependencies::ld_surrealdb::start_db_command;
use crate::first::new::first_time;
use local_ip_address::local_ip;
use surreal_db::db::DB;
use sysinfo::{Disks, System}; // we will need to check the disk usage here
pub fn start() {
    let os = System::name();
    if os.unwrap().as_str() == "Microsoft Windows" {
        println!("you are on Microsoft Windows she don't like that");
        Error::new(ErrorKind::Unsupported, "no Microsoft Windows support");
        exit(13); // it means the os is window and they out of luck
    };

    let connie_config = File::open("/.config/connie/connie_config.yaml");
    if connie_config.is_ok() {
        // let first_time = false;
        let ip = local_ip().expect("could no get ip");
        let ip = format!("{}", ip);
        start_db_command(ip.as_str());
        DB {
            addr: ip.as_str(),
            remote: false,
        };

        // let dependency_check = check_dependencies().is_ok();
        // if dependency_check == false {
        //     exit(2) //unmet dependency
        // }
    } else {
        let firs_time_state = first_time().expect("first_time process error");
        println!("process finished with {}", firs_time_state);
    };
}

