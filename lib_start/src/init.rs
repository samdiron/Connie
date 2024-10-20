use crate::dependencies::ld_nix::nix_ld_check;
use crate::dependencies::ld_openssl::{openssl_cert, openssl_ld_check};
use crate::dependencies::ld_surrealdb::{start_db_command, surreal_ld_check};
use crate::first::new::first_time;
use local_ip_address::local_ip;
use rpassword::read_password;
use std::fs::{remove_file,File};
use std::io::{stdout, Error, ErrorKind, Read, Result, Write};
use std::process::exit;
use surreal_db::db::DB;
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

fn check_pid_lockfile() -> Result<i32> {
    println!("process: checking pid file lock");
    let e_bool = File::open("~/.config/connie/tmp/pid_file").is_ok();

    if e_bool == true{
        let mut pid_lock_file = String::new();
        let mut f = File::open("~/.config/connie/tmp/pid_file").unwrap();
        f.read_to_string(&mut pid_lock_file)?;
        let mut sys = System::new();
        sys.refresh_all();
        let mut is_it: i32 = 2;
        for pid in sys.processes() {
            if pid_lock_file.contains(pid) == true {
                *&mut is_it = 1;
                println!("there is a connie process already running");
            } else {
                *&mut is_it = 0;
                println!("process: finished checking pid file lock");
                remove_file("~/.config/tmp/pid_file").expect("TODO: panic message");
                let file = File::create_new("~/.config/tmp/pid_file")
                    .expect("error while creating a new pid_lock");
                create_pid(file);
            }
        }
        drop(sys);
        drop(pid_lock_file);

        Ok(is_it)
    } else {
        let file = File::create_new("~/.config/connie/tmp/pid_file")
            .expect("could not create new pid lock file");
        create_pid(file);
        Ok(0)

    }.expect("TODO: panic message");
    Ok(0)
}
pub async fn start() -> Result<LocalMachine> {
    let home_path = "~/Connie";
    //let rt = Builder::new_current_thread().enable_all().build().unwrap();

    let os = System::name();
    if os.unwrap().as_str() == "Microsoft Windows" {
        println!("you are on Microsoft Windows she don't like that");
        Error::new(ErrorKind::Unsupported, "no Microsoft Windows support");
        exit(13); // it means the os is window and they out of luck
    };

    let connie_config = File::open("/.config/connie/connie_config.yaml");
    if connie_config.is_ok() {
        // let first_time = false;
        let dependencies_surreal = surreal_ld_check(home_path);
        let dependencies_openssl = openssl_ld_check(home_path);
        let dependencies_nix = nix_ld_check(home_path);
        let dependencies_check = dependencies_surreal + dependencies_nix + dependencies_openssl;
        if dependencies_check != 0 {
            exit(6);
        }
        let ip = local_ip().expect("could no get ip");
        let ip = format!("{}", ip);
        openssl_cert(ip.as_str());
        start_db_command(ip.as_str());
        let db_conn = DB {
            addr: ip.as_str(),
            remote: false,
        };
        let _ = db_conn.connect().await;
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
        drop(connie_config);
        Ok(machine)
    } else {
        let firs_time_state = first_time().await.expect("first_time process error");
        println!("now will exit if you want to start rerun connie");
        // Err(e) -> {
        //     eprintln!("{}",e);
        // };
        exit(firs_time_state);
    }
    //drop(connie_config)
}
