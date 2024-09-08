//TODO do shit here
use sysinfo::System;

fn main () {
    println!("{}", System::host_name().unwrap());
}