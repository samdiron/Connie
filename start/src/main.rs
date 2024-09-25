use std::string::String;
use std::fs::File;
use std::io::{stdin, stdout, Error, ErrorKind, Write};
use std::process;
use std::process::exit;
use surreal_db::db::DBASE;
use sysinfo::Cpu;
use sysinfo::System;
use uuid::Uuid;

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
    let consent = consent.as_str().trim_ascii_end();
    if consent.to_lowercase() == "yes" {
        println!("setting up now :)");
    }
    else if consent.trim_ascii_end() == "y" {
        println!("setting up now :)");
    }
    else if consent.trim_ascii_end() == "dev" {
        println!("okay");
    } else {
        exit(1);
    }

    println!("process: creating file /.config/connie/connie_config.yaml");
    println!("//NOTE cant be more than 17 char or less than 3 it cant contain spaces");
    print!("server name: ");
    stdout().flush().unwrap();
    let mut server_name_string: String = String::new();
    stdin().read_line(&mut server_name_string);

    let server_name = server_name_string.trim_ascii_end();
    if server_name.chars().count() >= 17 {
        loop {
           stdin().read_line(&mut server_name_string);
            if server_name_string.trim_ascii_end().chars().count() <= 17 {
                break;
            }
            else if server_name_string.trim_ascii_end().chars().all(char::is_whitespace) {
                print!("can't have spaces");
            } else {
                println!("invalied");
            }
        }
    }
    if server_name_string.trim_ascii_end().chars().count() <= 3 {
        stdin().read_line(&mut server_name_string);
        loop {
            if server_name_string.trim_ascii_end().chars().count() >= 3 {
                break
            } else {
                println!("too short");
            }
        }
    };
    let mut max_client_string : String = String::new();
    print!("maximum clients connecting to the server at the same time: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut max_client_string);
    //
    let max_client = max_client_string.trim_ascii_end();
    //TODO the value enterd in config.yaml ^
    let is_max_client_number = max_client.chars().all(char::is_numeric);
    if is_max_client_number == false {
        println!("enter only numbers larger that 0");
        loop {
            print!("enter a number: ");
            stdout().flush().unwrap();
            stdin().read_line(&mut server_name_string);
            let is_max_client_number = max_client.chars().all(char::is_numeric);
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
    let mut status_string : String = String::new();
    let mut status_u8: u8 = 0;
    loop {
        stdin().read_line(&mut status_string);
        let status = status_string.trim_ascii_end();
        if status == "0" {
            *&mut status_u8 = 0;
            break
        }
        else if status == "1" {
           *&mut status_u8 = 1;
            break
        }
        else { println!("enter a valid number"); };
    };
    let server_status = status_u8;
    //TODO yaml value ^


    println!("finshed getting server's identity");
    println!("now getting the user's identity this will be the admin user for the server and a user to connect to other servers");
    //TODO input  = {password , name , username} data = {input, isadmin, id, uuid, + registration server uuid  }

    let mut name = String::new();
    let _ = loop {
        println!("name should be 3~20 charcters of any language ");
        print!("name: ");
        stdout().flush().unwrap();
        let mut name_string: String = String::new();
        stdin().read_line(&mut name_string);

        let name_str = name_string.trim_ascii_end().to_owned();
        let is_valied = is_valied_str(&name_str);
        if (name_str.chars().count() <= 20) && (name_str.chars().count() >= 3) && (is_valied == true) {
            *&mut name = name_str;
            break
        }
        else {
            println!("enter a valid name ");
        };
    };


    //TODO name before username
    let mut user_name = String::new();
    let _ = loop {
        println!("username should be no spaces 3~20 charcters of any language numbers punctuation ");
        print!("username: ");
        stdout().flush().unwrap();
        let mut user_name_string: String = String::new();
        stdin().read_line(&mut user_name_string).unwrap();

        let user_name_str = user_name_string.trim_ascii_end().to_owned();
        let is_valied = is_valied_str(&user_name_str);
        if (user_name_str.chars().count() <= 20) && (user_name_str.chars().count() >= 3) && (is_valied == true) {
            *&mut user_name = user_name_str;
            break
        }
        else {
            println!("enter a valid username ");
        };
    };
    let mut password = String::new();
    let _ = loop {
        println!("password can be 3~20 charcters and numbers punctuation ");
        print!("password: ");
        stdout().flush().unwrap();
        let mut password_string: String = String::new();
        stdin().read_line(&mut password_string);

        let password_str = password_string.trim_ascii_end().to_owned();
        let is_valied = is_valied_str(&password_str);
        if (password_str.chars().count() <= 20) && (password_str.chars().count() >= 3) && (is_valied==true) {
                *&mut user_name = password_str;
                break
        }
        else {
            println!("enter a valid name ");
        };
    };

    let config_make = process::Command::new("sh")
        .arg("touch")
        .arg("~/.config/connie/connie_config.yaml")
        .output()
        .expect("could not preform a shell command");
    let config = config_make;


}
fn is_valied_str(s : &String) -> bool {
    let numerics = s.chars().filter(|c| c.is_numeric()).count();
    let letters = s.chars().filter(|c| c.is_alphabetic()).count();
    let punc = s.chars().filter(|c| c.is_ascii_punctuation()).count();
    let length = s.chars().count();
    let total = numerics + letters + punc;
    if total == length {
        return true
    }
    else {return false};
}

