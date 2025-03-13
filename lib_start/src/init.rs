use std::process::exit;

use crate::{check_path::check_first_time, checks::valid_db_conf};



pub fn start() {
    let first_time_ok = check_first_time();
    if first_time_ok.is_ok() {
        let first_time = first_time_ok.unwrap();
        if 1 == first_time {
            println!("something wrong with /opt/Connie/conf/ files ");
            println!("will exit with code 1");
            exit(1)
        }
    }
    else {
        let _ = valid_db_conf().unwrap();
    }

}
