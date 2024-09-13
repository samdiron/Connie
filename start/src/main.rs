use sysinfo::Cpu;
use sysinfo::System;
use surreal_db::db::DBASE;
use std::fs::File;
fn main() {
    let mut connie_config = File::open("/.config/connie/connie_config.yaml");
    if connie_config.unwrap() > 0 {
        let firsttime = false;
    }
    else {
        firstTime();
    }

    println!("Hello, world!");
}


fn firstTime() {
    println!("hi");
}