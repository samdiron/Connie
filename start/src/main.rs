use sysinfo::Cpu;
use sysinfo::System;
use surreal_db::db::DBASE;
use std::fs::File;
use std::io::stdin;
use std::process::exit;
use std::process;

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
        firstTime();
    }

}


fn firstTime() {
    let mut s = String::new();
    println!("thanks for chosing connie : she needs consent to make a few actions press (yes/no): ");
    let not_rape = stdin().read_line(&mut s);
    let binding = not_rape.unwrap().to_string().to_lowercase();
    let consent = binding.as_str();
    if  consent == "yes" {
        println!("setting up now :)");
    }if  consent== "y" {
        println!("setting up now :)");
    }
    else {
        println!("okay will abort");
        process::abort();
    }
    let config_make = process::Command::new("sh")
        .arg("touch")
        .arg("/.config/")
        .arg("connie/")
        .arg("connie_config.yaml")
        .output()
        .expect("colud not preform a shell command");
    let config = config_make.stdout;
    println!("process: creating a /.config/connie/connie_config.yaml");

    println!("")

}