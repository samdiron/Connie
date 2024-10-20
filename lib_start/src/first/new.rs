use crate::dependencies::{
    //dependency_fn_check,
    ld_openssl::openssl_cert,
    ld_surrealdb::start_db_command,
};
use local_ip_address::local_ip;
use rpassword::read_password;
use std::fs::{create_dir, File};
use std::io::{stdin, stdout, Write};
use std::process::exit;
use surreal_db::server::structs::Hardware;
use surreal_db::{server::structs::LocalMachine, user::sign_up::DUser};
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
        println!("setting up now :)");
    } else if consent.trim_ascii_end() == "dev" {
        println!("okay");
    } else {
        exit(1);
    }
    println!("process: creating ~/.config/connie");
    create_dir("~/.config/connie")?;

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
    let mut status_u8: u8 = 0;
    loop {
        stdin().read_line(&mut status_string).expect("1");
        let status = status_string.trim_ascii_end();
        if status == "0" {
            *&mut status_u8 = 0;
            break;
        } else if status == "1" {
            *&mut status_u8 = 1;
            break;
        } else {
            println!("enter a valid number");
        };
    }
    let server_status = status_u8;
    //TODO yaml value ^
    let mut server_password = String::new();
    loop {
        println!("password can be 3~20 characters and numbers punctuation ");
        print!("server password: ");
        stdout().flush().unwrap();
        let password_string = read_password().unwrap(); //String::new();
        let password_str = password_string.trim_ascii_end().to_owned();
        let is_valid = is_valid_str(&password_str);
        if (password_str.chars().count() <= 20)
            && (password_str.chars().count() >= 3)
            && (is_valid == true)
        {
            //
            print!("Confirm password: ");
            stdout().flush().unwrap();
            let paswd_confirm = read_password().unwrap();
            if paswd_confirm == password_str {
                *&mut server_password = password_str;
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

    let mut name = String::new();
    let _ = loop {
        println!("name should be 3~20 characters of any language ");
        print!("name: ");
        stdout().flush().unwrap();
        let mut name_string: String = String::new();
        let _ = stdin().read_line(&mut name_string);

        let name_str = name_string.trim_ascii_end().to_owned();
        let is_valid = is_valid_str(&name_str);
        if (name_str.chars().count() <= 20) && (name_str.chars().count() >= 3) && (is_valid == true)
        {
            *&mut name = name_str;
            break;
        } else {
            println!("enter a valid name ");
        };
    };

    //TODO name before username
    let mut user_name = String::new();
    let _ = loop {
        println!(
            "username should be no spaces 3~20 characters of any language numbers punctuation "
        );
        print!("username: ");
        stdout().flush().unwrap();
        let mut user_name_string: String = String::new();
        let _ = stdin().read_line(&mut user_name_string).unwrap();

        let user_name_str = user_name_string.trim_ascii_end().to_owned();
        let is_valid = is_valid_str(&user_name_str);
        if (user_name_str.chars().count() <= 20)
            && (user_name_str.chars().count() >= 3)
            && (is_valid == true)
        {
            *&mut user_name = user_name_str;
            break;
        } else {
            println!("enter a valid username ");
        };
    };
    let mut user_password = String::new();
    loop {
        println!("password can be 3~20 characters and numbers punctuation ");
        print!("password: ");
        stdout().flush().unwrap();
        let password_string = read_password().unwrap(); //String::new();
        let password_str = password_string.trim_ascii_end().to_owned();
        let is_valid = is_valid_str(&password_str);
        if (password_str.chars().count() <= 20)
            && (password_str.chars().count() >= 3)
            && (is_valid == true)
        {
            //
            print!("Confirm password: ");
            stdout().flush().unwrap();
            let paswd_confirm = read_password().unwrap();
            if paswd_confirm == password_str {
                *&mut user_password = password_str;
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
    let mut available_storage: u64 = 1;
    for disk in &disks {
        let ds = disk.available_space();
        let dps = ds + &available_storage;
        *&mut available_storage = dps;
    }
    let core_count = sys
        .physical_core_count()
        .expect("could not read core count");

    //let start_db = Command::new("sh").arg("surreal").arg("start");

    let ip = local_ip().expect("could not get ip to start db ");
    let str_ip = format!("{ip}");
    openssl_cert(str_ip.as_str());

    start_db_command(str_ip.as_str());

    let machine = LocalMachine {
        cpid: server_uuid,
        passwd: server_password,
        host_name: host_name.clone(),
        status: server_status,
        // max_client: max_clients,
        hardware: Hardware {
            swap: machine_swap,
            cpu_core_count: core_count,
            memory: machine_memory,
        },
    };
    machine.create().await.expect("TODO: panic message");

    let admin = DUser {
        is_admin: true,
        name,
        user_name,
        pass: user_password,
        cpid: admin_uuid,
    };

    let user_token = admin.sign_up_admin().await.expect("could not get token");
    //.expect("could not get user token");
    let data = format!("{},\n", user_token);
    let mut file = File::create_new("~/Connie/tmp/admin_jwt.csv").expect("could not create file");
    file.write_all(data.as_bytes())
        .expect("could not write data to file");

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
    let mut file = File::create("~/.config/connie/connie_config.yaml")
        .expect("could not create connie_config.yaml");
    file.write_all(yaml_config.as_bytes())
        .expect("could not write to connie_config.yaml");
    Ok(0)
}

fn is_valid_str(s: &String) -> bool {
    let numerics = s.chars().filter(|c| c.is_numeric()).count();
    let letters = s.chars().filter(|c| c.is_alphabetic()).count();
    let punc = s.chars().filter(|c| c.is_ascii_punctuation()).count();
    //let num = s.chars().all(|c| c.is_ascii_digit()).count();
    let length = s.chars().count();
    let total = numerics + letters + punc;
    return if total == length {
        true
    } else {
        false
    }
}
