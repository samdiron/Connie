use crate::first::new::first_time;
use common_lib::path::get_config_path;
use local_ip_address::local_ip;
use std::fs::{remove_file, File};
use std::io::{Error, ErrorKind, Read, Result, Write};
use std::process::exit;
use surreal_db::db::DBC;
use surreal_db::server::structs::{start_minfo, LocalMachine};
use sysinfo::get_current_pid;
use sysinfo::System; //{Disks, System}; // we will need to check the disk usage here

fn create_pid(mut f: File) {
    println!("process: finished checking pid file lock");
    let current = get_current_pid()
        .expect("could not get current pid")
        .to_string();
    f.write_all(current.as_bytes()).unwrap();
    f.flush().unwrap();
}

pub fn check_pid_lockfile() {
    // let cp = get_config_path();
    let full_path = "/Connie/tmp/lockfile";
    let mut e_bool: bool = File::open(full_path).is_ok();
    if File::open(full_path).is_err_and(|e| e.kind() == ErrorKind::NotFound) {
        e_bool = false
    };

    if e_bool {
        let mut lock_file = String::new();
        let mut f = File::open(full_path).unwrap();
        f.read_to_string(&mut lock_file).expect("exp");
        let mut sys = System::new();
        sys.refresh_all();
        for pid in sys.processes() {
            if lock_file.as_str() == (pid.0.to_string().as_str()) {
                println!("another connie process is running .");
                println!("the lockfile: {}", lock_file);
                println!("pid: {}; will exit now :( .", pid.0);
                exit(1)
            } else {
                remove_file(full_path).expect("TODO: panic message");
                let file =
                    File::create_new(full_path).expect("error while creating a new pid_lock");
                create_pid(file);
            }
        }
        drop(sys);
        drop(lock_file);
    } else {
        let file = File::create_new(full_path).expect("could not create new pid lock file");
        create_pid(file);
    };
}

pub async fn start() -> Result<()> {
    let cp = get_config_path();
    println!("{}", cp);
    let os = System::name();
    if os.unwrap().as_str() == "Microsoft Windows" {
        println!("you are on Microsoft Windows she don't like that");
        Error::new(ErrorKind::Unsupported, "no Microsoft Windows support");
        exit(13); // it means the os is window and they out of luck
    };
    let cdp = format!("{cp}connie_config.yaml");
    //TODO:
    let connie_config_file = File::open(cdp);
    let connie_config = connie_config_file.is_ok();
    drop(connie_config_file);
    if connie_config {
        let ip = local_ip().expect("could no get ip");
        let ip = format!("{}", ip);
        Ok(())
    } else {
        let firs_time_state = first_time().await.expect("first_time process error");
        println!("now will exit if you want to start rerun connie");
        exit(firs_time_state)
    }
}
