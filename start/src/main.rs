use sysinfo::Cpu;
use sysinfo::System;
use surreal_db::db::DBASE;
use std::fs::File;
use std::process::exit;
use std::process;
use std::io::{Error, ErrorKind, stdout, stdin, Write};


fn main() {
    let os = System::name();
    if os.unwrap().as_str() == "Microsoft Windows" {
        println!("you are on Microsoft Windows she don't like that");
        exit(13); // it mean the os is window and they out of luck
    };

    let mut connie_config = File::open("/.config/connie/connie_config.yaml");
    if connie_config.is_ok(){
        let firsttime = false;
    }
    else {
        firstTime()
    };

}


fn firstTime() {
    print!("thanks for chosing connie she needs consent to make a few actions (yes/no): ");
    stdout().flush().unwrap();
    let mut consent = String::new();
    stdin().read_line(&mut consent);
    //let binding = not_rape.unwrap().to_string().to_lowercase();
    let consent = consent.as_str();
    if  consent.trim_ascii_end().to_lowercase() == "yes" {
        println!("setting up nowdev :)");
    }
    if  consent.trim_ascii_end().to_lowercase() == "y" {
        println!("setting up now :)");
    }
    if consent.trim_ascii_end() == "dev" {
            println!("okay")
    }
    else {
        exit(1);
    }
    let config_make = process::Command::new("sh")
        .arg("touch")
        .arg("~/.config/connie/connie_config.yaml")
        //.arg("connie/")
        //.arg("connie_config.yaml")
        .output()
        .expect("colud not preform a shell command");
    let config = config_make;
    println!("process: creating file /.config/connie/connie_config.yaml");
    println!("//NOTE cant be more than 17 char or less than 3 it cant contain spaces");
    print!("name: ");
    stdout().flush().unwrap();
    let mut server_name_string : String = String::new();
    stdin().read_line(&mut server_name_string);

    let server_name = server_name_string.trim_ascii_end();
    if server_name.len() >= 16 {
        loop {
            stdin().read_line(&mut server_name_string);
            if server_name_string.trim_ascii_end().len() <= 16 {
                break
            }
            if server_name_string.trim_ascii_end().len() <= 2 {
                    println!("too short");
            }
            if server_name_string.trim_ascii_end().contains(" ") {
                    print!("can't have spaces");
            }
            else {
                println!("you are a dumb fuck; 16 or less");
                print!("server name: ");stdout().flush().unwrap();
                stdin().read_line(&mut server_name_string);
            }
            println!("how hard is it to enter a name that's more than 3 char less than 17;");

        }
    }



}