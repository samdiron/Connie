#![allow(clippy::if_same_then_else)]
use crate::{
    common::path::{c_path, h_path},
    dependencies::ld_openssl::openssl_cert,
};
use local_ip_address::local_ip;
use rpassword::read_password;
use std::fs::{create_dir, create_dir_all, exists, File};
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;
use std::process::exit;
use surreal_db::{db::first_time_db_def, server::structs::Hardware};
use surreal_db::{db::DBC, server::structs::LocalMachine, user::sign_up::User};
use sysinfo::{Disks, System};
use uuid::Uuid;

pub async fn first_time() -> std::io::Result<i32> {
    //let _ = dependency_fn_check();
    print!("do you want to setup Connie (yes/no): ");
    stdout().flush().unwrap();
    let mut consent = String::new();
    let _ = stdin().read_line(&mut consent);
    let consent = consent.as_str().trim_ascii_end();
    if consent.to_lowercase() == "yes" {
        println!("setting up now :)");
    } else if consent.trim_ascii_end() == "y" {
        println!("okay setting up now :)");
    } else if consent.trim_ascii_end() == "dev" {
        println!("okay");
    } else {
        exit(1);
    }
    println!("process: creating ~/.config/connie");
    let c_dir = c_path();
    let config_path = c_dir.as_str();
    let check = exists(config_path).expect("i have nothing");
    let mut path_tmp = PathBuf::new();
    path_tmp.push(config_path);
    path_tmp.push("/tmp");
    let mut home_path = PathBuf::new();
    let pstring = h_path();
    let ring = format!("{pstring}/Connie");
    home_path.push(ring.as_str());
    // home_path.push("/Connie");
    let check_home = home_path.exists();
    let check_tmp = exists(path_tmp.clone()).expect("could not check config/tmp");
    if !check_home {
        // println!("home : {}",pstring.as_str());
        println!("aad {}", home_path.display());
        let str_ring = ring.as_str();
        let surreald = format!("{str_ring}/surreal");
        let certd = format!("{str_ring}/cert");
        let keyd = format!("{str_ring}/key");
        // certd.push(pstring.as_str());
        // certd.push("/cert");
        // keyd.push(pstring.as_str());
        // keyd.push("/key");
        // surreald.push(pstring.as_str());
        // surreald.push("/surreal");
        //
        println!("creating dir: {}", surreald.as_str());
        create_dir_all(surreald).unwrap();
        println!("creating dir: {}", certd.as_str());
        create_dir(certd).unwrap();
        println!("creating dir: {}", keyd.as_str());
        create_dir(keyd).unwrap();
    };
    if !check {
        // create_dir(config_path.as_str()).expect("could not create config dir");
        //let mut  path = PathBuf::new();

        create_dir_all(path_tmp).expect("TODO: panic message");
    } else if !check_tmp {
        create_dir_all(path_tmp).expect("TODO: panic message");
    };

    // TODO : run define db functions here
    //
    // ;
    let ip = local_ip().unwrap();
    let db_ip = format!("{ip}");
    openssl_cert(db_ip.as_str()).await;
    let db_conn = DBC {
        addr: None,
        remote: false,
        lm: true,
    };
    db_conn.connect().await;
    let db_define = first_time_db_def().await.is_ok();
    if db_define == true {
        println!("db connection established");
    } else {
        println!("error db sql query failed will exit with code 2021 ");
        exit(2021)
    }

    println!("process: creating config");
    println!("//NOTE cant be more than 17 char or less than 3 it cant contain spaces");
    print!("server name: ");
    stdout().flush().unwrap();
    let mut server_name_string: String = String::new();
    let _ = stdin().read_line(&mut server_name_string);

    let server_name = server_name_string.trim_ascii_end();
    if server_name.chars().count() >= 17 {
        loop {
            let _ = stdin().read_line(&mut server_name_string);
            if server_name_string.trim_ascii_end().chars().count() <= 17 {
                break;
            } else if server_name_string
                .trim_ascii_end()
                .chars()
                .all(|c| c.is_whitespace())
            {
                print!("can't have spaces");
            } else {
                println!("invalid");
            }
        }
    }
    if server_name_string.trim_ascii_end().chars().count() <= 3 {
        let _ = stdin().read_line(&mut server_name_string);
        loop {
            if server_name_string.trim_ascii_end().chars().count() >= 3 {
                break;
            } else {
                println!("too short");
            }
        }
    };
    // let mut max_client_string: String = String::new();
    // print!("//note it can't be bigger than 255: maximum clients connecting to the server at the same time: ");
    // stdout().flush().unwrap();
    // let _ = stdin().read_line(&mut max_client_string);
    // let mut max_clients: u32 = 0;
    // let max_client_allowed = max_client_string.trim_ascii_end();
    // //TODO the value for in config.yaml ^
    // let is_max_client_number = max_client_allowed.chars().all(|c| c.is_ascii_digit());
    // if is_max_client_number == false {
    //     println!("enter only numbers larger that 0");
    //     loop {
    //         print!("enter a number: ");
    //         stdout().flush().unwrap();
    //         let _ = stdin().read_line(&mut max_client_string);
    //         let is_max_client_number = max_client_allowed.chars().all(|c| c.is_ascii_digit());
    //         if is_max_client_number {
    //             *&mut max_clients = max_client_allowed.parse().unwrap();
    //             break;
    //         } else {
    //             println!("are we really doing this ");
    //         }
    //     }
    // } else {
    //     *&mut max_clients = max_client_allowed.parse().unwrap();
    // };
    //
    println!("server: 0 ");
    println!("client & server: 1");
    print!("choose a server status(0/1): ");
    stdout().flush().unwrap();
    let mut status_string: String = String::new();
    let status_u8: u8;
    loop {
        stdin().read_line(&mut status_string).expect("1");
        let status = status_string.trim_ascii_end();
        if status == "0" {
            status_u8 = 0;
            break;
        } else if status == "1" {
            status_u8 = 1;
            break;
        } else {
            println!("enter a valid number");
        };
    }
    let server_status = status_u8;
    //TODO yaml value ^
    let server_password: String; // = String::new();
    loop {
        println!("password can be 3~20 characters and numbers punctuation ");
        print!("server password: ");
        stdout().flush().unwrap();
        let password_string = read_password().unwrap(); //String::new();
        let password_str = password_string.trim_ascii_end().to_owned();
        let is_valid = is_valid_str(&password_str);
        if (password_str.chars().count() <= 20) && (password_str.chars().count() >= 3) && is_valid {
            //
            print!("Confirm password: ");
            stdout().flush().unwrap();
            let paswd_confirm = read_password().unwrap();
            if paswd_confirm == password_str {
                server_password = password_str;
                break;
            } else {
                println!("password do not match");
            };
        } else {
            println!("enter a valid name ");
        };
    }
    println!("finished getting server's identity");
    println!("now getting the user's identity this will be the admin user for the server and a user to connect to other servers");
    //TODO input  = {password , name , username} data = {input, id, uuid, + registration server uuid  }

    let name: String; //= String::new();
    loop {
        println!("name should be 3~20 characters of any language ");
        print!("name: ");
        stdout().flush().unwrap();
        let mut name_string: String = String::new();
        let _ = stdin().read_line(&mut name_string);

        let name_str = name_string.trim_ascii_end().to_owned();
        let is_valid = is_valid_str(&name_str);
        if (name_str.chars().count() <= 20) && (name_str.chars().count() >= 3) && is_valid {
            name = name_str;
            break;
        } else {
            println!("enter a valid name ");
        };
    }

    //TODO name before username
    let user_name: String; //
    loop {
        println!(
            "username should be no spaces 3~20 characters of any language numbers punctuation "
        );
        print!("username: ");
        stdout().flush().unwrap();
        let mut user_name_string: String = String::new();
        let _ = stdin().read_line(&mut user_name_string).unwrap();

        let user_name_str = user_name_string.trim_ascii_end().to_owned();
        let is_valid = is_valid_str(&user_name_str);
        if (user_name_str.chars().count() <= 20) && (user_name_str.chars().count() >= 3) && is_valid
        {
            user_name = user_name_str;
            break;
        } else {
            println!("enter a valid username ");
        };
    }
    let user_password: String;
    loop {
        println!("password can be 3~20 characters and numbers punctuation ");
        print!("password: ");
        stdout().flush().unwrap();
        let password_string = read_password().unwrap(); //String::new();
        let password_str = password_string.trim_ascii_end().to_owned();
        let is_valid = is_valid_str(password_str.as_str());
        if (password_str.chars().count() <= 20) && (password_str.chars().count() >= 3) && is_valid {
            //
            print!("Confirm password: ");
            stdout().flush().unwrap();
            let paswd_confirm = read_password().unwrap();
            if paswd_confirm == password_str {
                user_password = password_str;
                break;
            } else {
                println!("password do not match");
            };
        } else {
            println!("enter a valid password ");
        };
    }
    let server_uuid = Uuid::new_v4();
    let admin_uuid = Uuid::new_v4();
    let mut sys = System::new_all();
    sys.refresh_all();
    let host_name = System::host_name().expect("string convert failed");
    let machine_memory = sys.total_memory();
    let machine_swap = sys.total_swap();
    let disks = Disks::new_with_refreshed_list();
    let mut available_storage: u64 = 0;
    for disk in &disks {
        let ds = disk.available_space();
        let dps = ds + available_storage;
        available_storage = dps;
    }
    let core_count = sys
        .physical_core_count()
        .expect("could not read core count");

    // //
    // let ip = local_ip().expect("could not get ip to start db ");
    // let str_ip = format!("{ip}");
    //

    // let database_init_conn = DB {
    //     addr: str_ip.as_str(),
    //     remote: false,
    // };
    // database_init_conn.connect().await.expect("could not connect to db");
    // // let _db = DBASE.clone();
    //

    let host2 = host_name.clone();
    let machine = LocalMachine {
        cpid: server_uuid,
        passwd: server_password,
        host_name: host2,
        status: server_status,
        // max_client: max_clients,
        server_name: server_name_string,
        hardware: Hardware {
            swap: machine_swap,
            cpu_core_count: core_count,
            memory: machine_memory,
        },
    };

    machine.create().await.expect("TODO: panic message");

    let admin = User {
        name,
        user_name,
        pass: user_password,
        cpid: admin_uuid,
    };

    admin.sign_up_A().await.expect("could not get token");

    let yaml_config = format!(
        "
        machine:
          - Host_name: {host_name}
          - Memory: {machine_memory}
          - Swap: {machine_swap}
          - Storage: {available_storage}
          - Cores: {core_count}
        "
    );
    println!("{}", yaml_config);
    let config_yml = format!("{config_path}/connie_config.yaml");
    let mut file = File::create(config_yml.as_str()).expect("could not create connie_config.yaml");
    file.write_all(yaml_config.as_bytes())
        .expect("could not write to connie_config.yaml");
    Ok(0)
}

fn is_valid_str(s: &str) -> bool {
    let numerics = s.chars().filter(|c| c.is_numeric()).count();
    let letters = s.chars().filter(|c| c.is_alphabetic()).count();
    let punc = s.chars().filter(|c| c.is_ascii_punctuation()).count();
    //let num = s.chars().all(|c| c.is_ascii_digit()).count();
    let length = s.chars().count();
    let total = numerics + letters + punc;
    return total == length;
}
