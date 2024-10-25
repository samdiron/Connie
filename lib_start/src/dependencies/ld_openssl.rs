
use std::fs::{create_dir_all, exists, remove_file, File};
use std::io::Write;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::process::Command;
use crate::common::path::h_path;
use crate::common::path::c_path;



pub fn openssl_ld_check(home_path: &str) -> u8 {

    println!("process: OpenSSL Check");
    let openssl_check = Command::new("sh")
        .arg("openssl")
        .arg("--version")
        .output()
        .is_ok();
    //.expect("ERROR: could not run openssl --version");
    // we will pretend that we have a version requirement of
    // OpenSSL 3.3.0
    return if openssl_check {
        0
    } else {
        let error_data = format!(
            "{},\n",
            Error::new(ErrorKind::NotFound, "OpenSSL not found")
        );
        println!("{}", error_data);
        let path = format!("{}/logs.csv", home_path);
        let mut file = File::open(path).expect("could not open logs.csv");
        file.write_all(error_data.as_bytes())
            .expect("could not write to logs.csv");
        1
    }
}

pub async fn open_command() -> i32{
    let cp = c_path();
    let hp = h_path();
    let san_p = format!("{}/tmp/san.cnf",cp.as_str());
    let cert_p = format!("{}/Connie/cert/cert.pem",hp.as_str());
    let key_p = format!("{}/Connie/cert/cert.pem",hp.as_str());
    let command = Command::new("sh")
        .arg("openssl")
        .arg("req")
        .arg("-x509")
        .arg("-nodes")
        .arg("-days")
        .arg("730")
        .arg("-newkey")
        .arg("rsa:2048")
        .arg("-keyout")
        .arg(key_p)
        .arg("-out")
        .arg(cert_p)
        .arg("-config")
        .arg(san_p)
        .output()
        .is_ok();//expect("failed to run openssl req command ");
    return if command {
       0 
    }else {
        1
    };
    // println!("process: created openssl certificate ");
    
}

pub async fn openssl_cert(ip: &str) -> i32 {
    let cp = c_path();
    println!("process: creating openssl certificate");
    let path = format!("{cp}/tmp/san.cnf");
    let dir_p = format!("{cp}/tmp");
    let mut  pathbuf = PathBuf::new();
    pathbuf.push(dir_p);
    let check = pathbuf.exists();
    if !check {
        create_dir_all(pathbuf.clone()).expect("could not create dir");
        pathbuf.push(path.as_str());
    }
    let data = format!(
        "
  [req]
  distinguished_name = req_distinguished_name
  req_extensions = v3_req
  prompt = no
  [req_distinguished_name]
  CN = No-Domain Server
  stateOrProvinceName = N/A
  localityName = N/A
  organizationName = Connie_server
  commonName = {i}: Self-signed certificate
  [v3_req]
  subjectAltName = @alt_names
  [alt_names]
  IP.1 = {i}",
        i = ip
    );
    println!("creating {}", path.as_str());
    let exist = exists(path.as_str()).unwrap();
    if exist {
        remove_file(path.as_str()).expect("could not remove old san.cnf");
    }
    let mut f = File::create(path.as_str()).expect("could not create a openssl tls config cert");
    f.write_all(data.as_bytes()).expect("could not write data to req config");
    let cert_state = open_command().await;
    println!("cert state = {}",cert_state);
    return cert_state
    
}
