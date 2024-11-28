use crate::common::path::{c_path, h_path};
use crate::dependencies::ld_nix::nix_ld_check;
use crate::dependencies::ld_openssl::{openssl_cert, openssl_ld_check};
use crate::dependencies::ld_surrealdb::{start_db_command, surreal_ld_check};
use crate::first::new::first_time;
use local_ip_address::local_ip;
use multicast::cast::cast_and_buffer;
use rpassword::read_password;
use std::fs::{remove_file, File};
use std::io::{stdout, Error, ErrorKind, Read, Result, Write};
use std::net::IpAddr;
use std::process::exit;
use std::str::FromStr;
use surreal_db::server::structs::{start_minfo, LocalMachine};
use sysinfo::get_current_pid;
use sysinfo::System; //{Disks, System}; // we will need to check the disk usage here
                     //use tokio::runtime::Builder;

fn create_pid(mut f: File) {
    println!("process: finished checking pid file lock");
    let current = get_current_pid()
        .expect("could not get current pid")
        .to_string();
    f.write_all(current.as_bytes()).unwrap();
    f.flush().unwrap();
}

pub fn check_pid_lockfile() -> i32 {
    let cp = c_path();
    let full_path = format!("{cp}/tmp/pid_file");
    println!("process: checking pid file lock");
    let mut e_bool: bool = File::open(full_path.as_str()).is_ok();
    if File::open(cp).is_err_and(|e| e.kind() == ErrorKind::NotFound) {
        e_bool = true
    }

    return if e_bool {
        let mut pid_lock_file = String::new();
        let mut f = File::open(full_path.as_str()).unwrap();
        f.read_to_string(&mut pid_lock_file).expect("exp");
        let mut sys = System::new();
        sys.refresh_all();
        let mut is_it: i32 = 2;
        for pid in sys.processes() {
            if pid_lock_file.contains(pid.0.to_string().as_str()) {
                is_it = 0;
                println!("there is a connie process already running");
            } else {
                is_it = 0;
                println!("process: finished checking pid file lock");
                remove_file(full_path.as_str()).expect("TODO: panic message");
                let file = File::create_new(full_path.as_str())
                    .expect("error while creating a new pid_lock");
                create_pid(file);
            }
        }
        drop(sys);
        drop(pid_lock_file);

        is_it
    } else {
        let file =
            File::create_new(full_path.as_str()).expect("could not create new pid lock file");
        create_pid(file);
        0
    };
}

pub async fn start() -> Result<LocalMachine> {
    let cp = c_path();
    println!("{}", cp);
    let hp = h_path();
    let home_path = format!("{hp}/Connie");

    let os = System::name();
    if os.unwrap().as_str() == "Microsoft Windows" {
        println!("you are on Microsoft Windows she don't like that");
        Error::new(ErrorKind::Unsupported, "no Microsoft Windows support");
        exit(13); // it means the os is window and they out of luck
    };
    let cdp = format!("{cp}/connie_config.yaml");
    let connie_config = File::open(cdp).is_ok();
    if connie_config {
        // let first_time = false;
        let dependencies_surreal = surreal_ld_check(home_path.as_str());
        let dependencies_openssl = openssl_ld_check(home_path.as_str());
        let dependencies_nix = nix_ld_check(home_path.as_str());
        let dependencies_check = dependencies_surreal + dependencies_nix + dependencies_openssl;
        if dependencies_check != 0 {
            exit(6);
        }
        let ip = local_ip().expect("could no get ip");
        let ip = format!("{}", ip);
        let _os = openssl_cert(ip.as_str()).await;
        let _ds = start_db_command(ip.as_str()).await;
        // let db_conn = DB {
        //     addr: ip.as_str(),
        //     remote: false,
        // };
        // let _ = db_conn.connect().await;
        let machine = start_minfo().await.expect("could not get machine info ");
        let passwd = machine.passwd.clone();
        let mut i = 0;
        while i <= 2 {
            print!("Enter Connie password");
            stdout().flush().unwrap();
            // let mut check_passwd = String::new();
            let check_passwd = read_password().unwrap();
            if check_passwd.trim_ascii_end() == passwd {
                println!("Okay: Start");
                break;
            } else {
                println!("try again");
                i += 1;
                if i > 2 {
                    exit(4);
                }
            }
        }

        // let cast_ip = IpAddr::from_str(ip.as_str()).expect("TODO : ip str to addr msg");

        // let _ = cast_and_buffer(cast_ip, 0).await;

        Ok(machine)
    } else {
        let firs_time_state = first_time().await.expect("first_time process error");
        println!("now will exit if you want to start rerun connie");
        // Err(e) -> {
        //     eprintln!("{}",e);
        // };
        exit(firs_time_state)
    }
    //drop(connie_config)
}
