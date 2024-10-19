use crate::dependencies::ld_nix::nix_ld_check;
use crate::dependencies::ld_openssl::{openssl_cert, openssl_ld_check};
use crate::dependencies::ld_surrealdb::{start_db_command, surreal_ld_check};
use crate::first::new::first_time;
use local_ip_address::local_ip;
//use rpassword::read_password;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::process::exit;
use surreal_db::db::DB;
use surreal_db::server::structs::{start_minfo, LocalMachine};
use sysinfo::System; //{Disks, System}; // we will need to check the disk usage here
use tokio::runtime::Builder;
#[tokio::main]
pub async fn start() {
    let home_path = "~/Connie";
    let rt = Builder::new_current_thread().enable_all().build().unwrap();

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
        if dependencies_check > 0 {
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
        db_conn.connect().await;
        let machine: LocalMachine = start_minfo().unwrap().await;
        let passwd = machine.passwd;
    } else {
        let firs_time_state = rt.block(first_time()).expect("first_time process error");
        println!("now will exit if you want to start rerun connie");
        exit(firs_time_state);
    };
}
