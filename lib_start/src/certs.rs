use std::{fs::File, io::{Result, Write}, process::Command, time::{self, SystemTime}};

use common_lib::{cheat_sheet::gethostname, path::CERTIFICATE_INFO};
use common_lib::toml;
use common_lib::serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize)]
struct CertFile {
    date: SystemTime,
    hostname: String,
}


fn update_cert_file() -> Result<()> {
    let mut f = File::open(CERTIFICATE_INFO)?;
    let time = time::SystemTime::now();
    let hostname = gethostname().to_str().unwrap().to_string();
    let new = CertFile {
        date: time,
        hostname
    };
    let tomled = toml::to_string(&new).unwrap();
    f.write_all(tomled.as_bytes())?;
    Ok(())
}

pub fn generate_certs() {
    let nc = Command::new("/opt/Connie/bin/openssl_script.sh")
        .spawn()
        .expect("could not start command");        
    nc.wait_with_output().expect("chiled process did not returened an error ");

    
}
