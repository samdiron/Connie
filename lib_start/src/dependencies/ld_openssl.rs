use std::fs::{File, remove_file};
use std::io::{Error, ErrorKind};
use std::io::Write;
use std::process::Command;

pub fn check(home_path: &str) -> u8 {
    println!("process: OpenSSL Check");
    let mut openssl_check = Command::new("sh")
        .args("openssl --version")
        .output().is_ok();
        //.expect("ERROR: could not run openssl --version");
    // we will pretend that we have a version requairment of
    // OpenSSL 3.3.0
    if openssl_check == true {
        return 0
    }
    else {
        let error_data = format!("{},\n",(Error::new(ErrorKind::NotFound , "OpenSSL not found"))).as_bytes();
        println!("{}",error_data);
        let path = format!("{}/logs.csv",home_path);
        let mut  file = File::open(path).expect("could not open logs.csv");
        file.write_all(error_data).expect("could not write to logs.csv");
        return 1
    }
}

pub fn openssl_cert(&ip: &str){
    let path = "~/.config/connie/tmp/san.cnf";
    let data = format!("
  [req]
  distinguished_name = req_distinguished_name
  req_extensions = v3_req
  prompt = no
  [req_distinguished_name]
  CN = No-Domain Server
  stateOrProvinceName = N/A
  localityName = N/A
  organizationName = Self-signed certificate
  commonName = {i}: Self-signed certificate
  [v3_req]
  subjectAltName = @alt_names
  [alt_names]
  IP.1 = {i}",i = ip);
    println!("creating {}",path);
    let exist = File::open(path).is_ok();
    if exist == true {
        remove_file(path);
    }
    let mut f = File::create(path).expect("could not create a openssl tls config cert");
    f.write_all(data.as_bytes()).expect("could not write data to req config");
}

pub fn open_command() {
    let mut create_tls_cert = Command::new("sh").arg("openssl")
        .arg("req")
        .arg("-x509")
        .arg("-nodes")
        .arg("-days")
        .arg("730")
        .arg("-newkey")
        .arg("rsa:2048")
        .arg("-keyout")
        .arg("~/Connie/key/key.pem")
        .arg("-out")
        .arg("~/Connie/cert/cert.pem")
        .arg("-config")
        .arg("~/.config/connie/tmp/san.cnf")
        .output()
        .expect("failed to run openssl req command ");


}
