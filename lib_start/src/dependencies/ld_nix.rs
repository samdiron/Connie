use std::fs::File;
use std::process::Command;
use std::io::{Error, ErrorKind, Write};

fn check(home_path : &str) -> u8 {
    let nix = Command::new("sh")
        .arg("nix")
        .arg("--version")
        .output().is_ok();
    if nix == true {
        return 0
    }
    else {
        let error_data = format!("{},\n",(Error::new(ErrorKind::NotFound , "nix package manager not found")));
        println!("{}",error_data);
        let path = format!("{}/logs.csv",home_path);
        let mut  file = File::open(path).expect("could not open logs.csv");
        file.write_all(error_data.as_bytes()).expect("could not write to logs.csv");
        return 1
    }

}
