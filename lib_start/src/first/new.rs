#![allow(clippy::if_same_then_else)]
// use common_lib::cheat_sheet::LOCAL_IP;
use common_lib::path::{get_config_path, get_home_path};
use lib_db::database::{get_conn, migrate};
use lib_db::server::server_struct::Server;
use lib_db::user::user_struct::User;
use lib_db::user::{self, user_struct};
use rpassword::read_password;
use std::fmt::{Debug, Formatter};
use std::fs::{create_dir, create_dir_all, exists, File};
use std::i64;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;
use std::process::{abort, exit};
use std::thread::available_parallelism;
use sysinfo::{Disks, System};

async fn fake_db_cred() {
    let pool = get_conn().await.unwrap();
    let _migrate = migrate(&pool).await.unwrap();
    let server = Server {
        cpid: String::new(),
        name: "name".to_string(),
        host: "host".to_string(),
        memory: 69,
        storage: 66,
        max_conn: 80,
        password: "hashed".to_string(),
    };
    let server = server.create(&pool).await.unwrap();
    assert_ne!(server.password, "hashed".to_string());
    let user = User {
        cpid: String::new(),
        name: "name".to_string(),
        host: server.cpid,
        username: "username".to_string(),
        email: "email".to_string(),
        password: "test".to_string(),
    };

    let user2 = user.create(&pool).await.unwrap();
    assert_ne!("test".to_string(), user2.password);
}

pub async fn first_time() -> std::io::Result<i32> {
    // let _ = dependency_fn_check();
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
    println!("process: creating /Connie/.config");
    let c_dir = get_config_path();
    let config_path = c_dir.as_str();
    let check = exists(config_path).expect("i have nothing");
    let mut path_tmp = PathBuf::new();
    path_tmp.push("/Connie/tmp");
    let mut home_path_buffer = PathBuf::new();
    let path_string = get_home_path();
    let home = path_string;
    home_path_buffer.push(home.as_str());
    let check_home = home_path_buffer.exists();
    let check_tmp = exists(path_tmp.clone()).expect("could not check config/tmp");
    let mut path_to_cpid_file: String = String::new();
    if !check_home {
        // println!("home : {}",pstring.as_str());
        println!("add {}", home_path_buffer.display());
        let str_home = home.as_str();
        let certd = format!("{str_home}/cert");
        let keyd = format!("{str_home}/key");
        let path_to_cpid = format!("{str_home}/etc");

        println!("creating dir: {}", certd.as_str());
        create_dir_all(certd).unwrap();
        println!("creating dir: {}", keyd.as_str());
        create_dir(keyd).unwrap();
        println!("creating dir: {}", path_to_cpid.as_str());
        create_dir(path_to_cpid.as_str()).unwrap();
        let path_cpid_file = format!("{path_to_cpid}/cpid");
        *&mut path_to_cpid_file = path_cpid_file;
    };
    if !check {
        create_dir_all(path_tmp).expect("TODO: panic message");
    } else if !check_tmp {
        create_dir_all(path_tmp).expect("TODO: panic message");
    };
    let pool = get_conn().await.unwrap();
    fake_db_cred().await;
    // let _migrate = migrate(&pool).await.unwrap();
    // let server = Server {
    //     cpid: String::new(),
    //     name: "name".to_string(),
    //     host: "host".to_string(),
    //     memory: 69,
    //     storage: i64::MAX,
    //     max_conn: 80,
    //     password: "hashed".to_string(),
    // };
    // assert_ne!(server.password, "hashed".to_string());
    // let user = User {
    //     cpid: String::new(),
    //     name: "name".to_string(),
    //     host: "host".to_string(),
    //     username: "username".to_string(),
    //     email: "email".to_string(),
    //     password: "test".to_string(),
    // };
    //
    // let user2 = user.create(&pool).await.unwrap();
    // assert_ne!("test".to_string(), user2.password);
    abort();
    println!("process: creating config");
    println!("//NOTE cant be more than 17 char or less than 3 it cant contain spaces");
    print!("server name: ");
    stdout().flush().unwrap();
    let mut server_name_string: String = String::new();
    let size = stdin().read_line(&mut server_name_string).unwrap();
    let server_name = server_name_string[..size].to_string();
    // let server_name = server_name_string.trim_ascii_end();
    // if server_name.chars().count() >= 17 {
    //     loop {
    //         let _ = stdin().read_line(&mut server_name_string);
    //         if server_name_string.trim_ascii_end().chars().count() <= 17 {
    //             break;
    //         } else if server_name_string
    //             .trim_ascii_end()
    //             .chars()
    //             .all(|c| c.is_whitespace())
    //         {
    //             print!("can't have spaces");
    //         } else {
    //             println!("invalid");
    //         }
    //     }
    // }
    // if server_name_string.trim_ascii_end().chars().count() <= 3 {
    //     let _ = stdin().read_line(&mut server_name_string);
    //     loop {
    //         if server_name_string.trim_ascii_end().chars().count() >= 3 {
    //             break;
    //         } else {
    //             println!("too short");
    //         }
    //     }
    // };

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

    let name: String;
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

    let user_name: String;
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
        let password_string = read_password().unwrap();
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

    let mut sys = System::new_all();
    sys.refresh_all();
    let host_name = System::host_name().expect("string convert failed");
    let machine_memory = sys.total_memory() as i64;
    let disks = Disks::new_with_refreshed_list();
    let mut available_storage: i64 = 0;
    for disk in &disks {
        let ds = disk.available_space() as i64;
        let dps = ds + available_storage;
        available_storage = dps;
    }

    let host2 = host_name.clone();

    let user_uuid = String::new();
    let server = Server {
        cpid: String::new(),
        name: server_name.to_string(),
        host: host_name.clone(),
        memory: machine_memory,
        storage: available_storage,
        max_conn: 80,
        password: server_password,
    };
    let server = server.create(&pool).await.unwrap();
    let yaml_config = format!(
        "
        machine:
          - Host_name: {host_name}
          - Memory: {machine_memory}
          - Storage: {available_storage}
        "
    );
    println!("{}", yaml_config);
    let config_yml = format!("{config_path}/config.yaml");
    let mut file = File::create(config_yml.as_str()).expect("could not create config.yaml");
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
