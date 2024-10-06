use std::fs::File;
use std::io::{stdin, stdout, Error, ErrorKind, Result, Write};
use std::process;
use std::process::{exit, Command};
use std::string::String;
use surreal_db::db::{DB, DBASE};
//use num_cpus;
use sysinfo::{Disks, System};
use uuid::Uuid;
//use log::error;
use local_ip_address::local_ip;
use rpassword::read_password;

fn main() {
    let os = System::name();
    if os.unwrap().as_str() == "Microsoft Windows" {
        println!("you are on Microsoft Windows she don't like that");
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
        firstTime()
    };
}

fn firstTime() {
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

    println!("process: creating file /.config/connie/connie_config.yaml");
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
    let mut max_client_string: String = String::new();
    print!("maximum clients connecting to the server at the same time: ");
    stdout().flush().unwrap();
    let _ = stdin().read_line(&mut max_client_string);
    //
    let max_client = max_client_string.trim_ascii_end();
    //TODO the value enterd in config.yaml ^
    let is_max_client_number = max_client.chars().all(|c| c.is_ascii_digit());
    if is_max_client_number == false {
        println!("enter only numbers larger that 0");
        loop {
            print!("enter a number: ");
            stdout().flush().unwrap();
            let _ = stdin().read_line(&mut server_name_string);
            let is_max_client_number = max_client.chars().all(|c| c.is_ascii_digit());
            if is_max_client_number {
                break;
            } else {
                println!("are we really doing this ");
            }
        }
    };
    println!("server: 0 ");
    println!("client & server: 1");
    print!("choose a server status(0/1): ");
    stdout().flush().unwrap();
    let mut status_string: String = String::new();
    let mut status_u8: u8 = 0;
    loop {
        stdin().read_line(&mut status_string);
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

    println!("finished getting server's identity");
    println!("now getting the user's identity this will be the admin user for the server and a user to connect to other servers");
    //TODO input  = {password , name , username} data = {input, id, uuid, + registration server uuid  }

    let mut name = String::new();
    let _ = loop {
        println!("name should be 3~20 characters of any language ");
        print!("name: ");
        stdout().flush().unwrap();
        let mut name_string: String = String::new();
        stdin().read_line(&mut name_string);

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
    let mut password = String::new();
    let _ = loop {
        println!("password can be 3~20 characters and numbers punctuation ");
        print!("password: ");
        stdout().flush().unwrap();
        let mut password_string = read_password().unwrap(); //String::new();
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
                *&mut password = password_str;
                break;
            } else {
                println!("password do not match");
            };
        } else {
            println!("enter a valid name ");
        };
    };
    let server_uuid = Uuid::new_v4();
    let admin_uuid = Uuid::new_v4();
    let mut sys = System::new_all();
    sys.refresh_all();
    let host_name = System::host_name();
    let memory = sys.total_memory();
    let swap = sys.total_swap();
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
    let startdb = Command::new("sh").arg("surreal").arg("start");
    let ip = local_ip().expect("could not get ip to start db ");
    let full_ip = format!("{}:8060", ip);
    let mut dbase_conniection = DB {
        addr: full_ip.as_str(),
        remote: false,
    };
    // let database = DBASE;
    // database.use_db("private_infer");
    // database.use_ns("machine_info");
    //input the info to the db.

    let yaml_config = ("
        machine:
          - Host_name: {host_name}
          - Meomory: {memory}
          - Swap: {swap}
          - Storage: {available_storage}
          - Cores: {core_count}
        ");
    println!("{}", yaml_config);

    //let config_make = process::Command::new("sh")
    //    .arg("touch")
    //     .arg("~/.config/connie/connie_config.yaml")
    //     .output()
    //     .expect("could not preform a shell command");
    //let config = config_make;
}
fn is_valid_str(s: &String) -> bool {
    let numerics = s.chars().filter(|c| c.is_numeric()).count();
    let letters = s.chars().filter(|c| c.is_alphabetic()).count();
    let punc = s.chars().filter(|c| c.is_ascii_punctuation()).count();
    //let num = s.chars().all(|c| c.is_ascii_digit()).count();
    let length = s.chars().count();
    let total = numerics + letters + punc;
    if total == length {
        return true;
    } else {
        return false;
    };
}
// fn is_not_sql_injection(s:&str){
//     let sql_words = ["","",];
//     s.contains()
// }
//new idea all string in db that come from user is converted to ascii to prevent sql injections
//it take more space but this in not a public server so it's about right

// fn check_dependencies() -> Result<T, Error>  {
//     let surreal_db_check = Command::new("sh")
//         .arg("surreal")
//         .arg("--version")
//         .output().expect("surrealdb check failed to start");
//     match surreal_db_check {
//         Ok(_) => {println!("surrealDB is Okay")}
//         Err(_) => {
//             println!("ERROR: surreal db not found");
//             //eprintln!("{}",);
//             Error::new(ErrorKind::NotFound ,"SurrealDB not found");
//             //eprintln!("{}",error)
//         }
//     }
//     let ffmpeg_check = Command::new("sh").arg("ffmpeg")
//         .output();
//     match ffmpeg_check {
//         Ok(_) => {println!("ffmpeg is Okay")},
//         Err(e) => {
//             println!("ERROR: ffmpeg not found");
//             return  Error::new(ErrorKind::NotFound, "FFmpeg not found");
//         }
//
//     }
//     let openssl_check = Command::new("sh").arg("openssl")
//         .arg("-sversion")
//         .output();
//     match openssl_check {
//         Ok(_) => {println!("OpenSSL is Okay")},
//         Err(e) => {
//             println!("ERROR: OpenSSL not found");
//             return  Error::new(ErrorKind::NotFound, "OpenSSL not found");
//         }
//
//     }
//
// }
