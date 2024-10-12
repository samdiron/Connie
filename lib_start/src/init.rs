use std::fs::File;
use std::io::{Error, ErrorKind};
use std::process::exit;
//use num_cpus;
use sysinfo::{Disks, System};
use crate::first::new::first_time;
pub fn start() {
    let os = System::name();
    if os.unwrap().as_str() == "Microsoft Windows" {
        println!("you are on Microsoft Windows she don't like that");
        Error::new(ErrorKind::Unsupported, "no Microsoft Eindows support");
        exit(13); // it mean the os is window and they out of luck
    };

    let mut connie_config = File::open("/.config/connie/connie_config.yaml");
    if connie_config.is_ok() {
        let firsttime = false;
        // let dependency_check = check_dependencies().is_ok();
        // if dependency_check == false {
        //     exit(2) //unmet dependency
        // }
    } else {
        first_time()
    };
}