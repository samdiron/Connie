use std::fs::File;
use std::io::{stdin, stdout, Error, ErrorKind, Write};
use std::mem::take;
use std::process;
use std::process::exit;
//use serde_yaml::Value::String;
use surreal_db::db::DBASE;
use sysinfo::Cpu;
use sysinfo::System;

fn main() {
    let os = System::name();
    if os.unwrap().as_str() == "Microsoft Windows" {
        println!("you are on Microsoft Windows she don't like that");
        exit(13); // it mean the os is window and they out of luck
    };

    let mut connie_config = File::open("/.config/connie/connie_config.yaml");
    if connie_config.is_ok() {
        let firsttime = false;
    } else {
        firstTime()
    };
}

fn firstTime() {
    print!("thanks for choosing connie she needs consent to make a few actions (yes/no): ");
    stdout().flush().unwrap();
    let mut consent = String::new();
    stdin().read_line(&mut consent);
    //let binding = not_rape.unwrap().to_string().to_lowercase();
    let consent = consent.as_str();
    if consent.trim_ascii_end().to_lowercase() == "yes" {
        println!("setting up nowdev :)");
    }
    if consent.trim_ascii_end().to_lowercase() == "y" {
        println!("setting up now :)");
    }
    if consent.trim_ascii_end() == "dev" {
        println!("okay")
    } else {
        exit(1);
    }

    println!("process: creating file /.config/connie/connie_config.yaml");
    println!("//NOTE cant be more than 17 char or less than 3 it cant contain spaces");
    print!("name: ");
    stdout().flush().unwrap();
    let mut server_name_string: String = String::new();
    stdin().read_line(&mut server_name_string);

    let server_name = server_name_string.trim_ascii_end();
    if server_name.len() >= 16 {
        loop {
           stdin().read_line(&mut server_name_string);
            if server_name_string.trim_ascii_end().len() <= 16 {
                break;
            }
            if server_name_string.trim_ascii_end().contains(" ") {
                print!("can't have spaces");
            } else {
                println!("you are a dumb fuck; 16 or less");
            }
            //println!("how hard is it to enter a name that's more than 3 char less than 17;");
        }
    }
    if server_name_string.trim_ascii_end().len() <= 2 {
        stdin().read_line(&mut server_name_string);
        loop {
            if server_name_string.trim_ascii_end().len() >= 3 {
                break
            } else {
                println!("too short");
            }
        }
    };
    let mut max_client_string : String = String::new();
    print!("maximum clients connecting to the server at the same time;");
    stdout().flush().unwrap();
    stdin().read_line(&mut max_client_string);
    //
    let max_client = max_client_string.trim_ascii_end();
    //the value enterd in config.yaml ^
    let is_max_client_number = max_client_string.chars().all(char::is_numeric);
    if is_max_client_number == false {
        println!("enter only numbers larger that 0");
        loop{
            stdin().read_line(&mut server_name_string);
        }
        if is_max_client_number {
            break;
        }
        else {
            println!("are we really doing this ");
        }
    };
    println!(
        "\
        server: 0 \
        client: 1 \
        client & server: 2
    ");
    print!("choose a server status(0/1/2): ");
    stdout().flush().unwrap();
    let mut status_string : String = String::new();
    // stdin().read_line(&mut status_string);
    // let status = status_string.trim_ascii_end();
    loop {
        stdin().read_line(&mut status_string);
        let status = status_string.trim_ascii_end();
        if status.chars().count() == 1 {
            break
        }
        else {
            println!("enter 1 number");
        };
        if  status.chars().all(char::is_numeric) == true {
            break
        }
        else {
            println!("enter number");
        };
        if status == "0" {
            break
        }
        if status == "1" {
            break
        }
        if status == "2" {
            break
        }
        else { println!("enter a valid number"); };
    }
    // if status.chars().count() > 1 {
    //     loop {
    //        stdin().read_line(&mut status_string);
    //         if status_string.trim_ascii_end().chars().count() == 1 {
    //             break
    //         }
    //         else {
    //             println!("enter 1 number");
    //         }
    //     }
    // }
    // if status.chars().all(char::is_numeric) == false {
    //    loop {
    //        stdin().read_line(&mut status_string);
    //         if  status_string.trim_ascii_end().chars().all(char::is_numeric) {
    //             break
    //         }
    //         else {
    //             println!("enter a number");
    //         }
    //     }
    // }



    // let max_client_char_is_num = max_client_string.chars()
    //     .map(|c|c.is_digit())
    //     .collect();
    // let is_maxclient_num = !max_client_char_is_num.contains(&false);
    //
    let config_make = process::Command::new("sh")
        .arg("touch")
        .arg("~/.config/connie/connie_config.yaml")
        .output()
        .expect("could not preform a shell command");
    let config = config_make;





}

