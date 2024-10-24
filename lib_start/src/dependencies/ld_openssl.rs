use std::fs::{remove_file, File, exists};
use std::io::Write;
use std::io::{Error, ErrorKind};
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
    return if openssl_check == true {
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

pub fn open_command() {
    let cp = c_path();
    let hp = h_path();
    let thp = hp.clone();
    let san_p = format!("{cp}/tmp/san.cnf");
    let cert_p = format!("{hp}/Connie/cert/cert.pem");
    let key_p = format!("{thp}/Connie/cert/cert.pem");
    Command::new("sh")
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
        .expect("failed to run openssl req command ");
    println!("process: created openssl certificate");
}

pub fn openssl_cert(ip: &str) {
    let cp = c_path();
    println!("process: creating openssl certificate");
    let path = format!("{cp}/tmp/san.cnf");
    // let dir_p = format!("{cp}/tmp");

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
    if exist == true {
        let _ = remove_file(path.as_str()).expect("could not remove old san.cnf");
    }
    let mut f = File::create_new(path.as_str()).expect("could not create a openssl tls config cert");
    f.write_all(data.as_bytes()).expect("could not write data to req config");
    open_command();
    println!("finished: openssl certificate successfully yay")
}
