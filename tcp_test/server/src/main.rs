use std::io::{Error, ErrorKind, Write};
use std::sync::Arc;
use rustls::pki_types::{CertificateDer, PrivateKeyDer };
use rustls::server::{Acceptor, NoClientAuth};
use rustls::{ConfigBuilder, ServerConfig};
use std::io::{self, BufReader};
use std::fs::File;
use rustls::crypto::CryptoProvider;
//use rcgen::Certificate;
use rustls_pemfile;

//static CERT : Vec<CertificateDer> = load_cert("~/.config/connie/certificates/cert.pem");
//static PRIVATE_KEY : Vec<PrivateKeyDer> = load_private_certificate_key("~/.config/connie/keys/key.pem");
// const CERT : io::Result<Vec<CertificateDer>> = load_cert("~/.config/connie/certificates/cert.pem");
// const PRIVATE_KEY : io::Result<PrivateKeyDer> = load_private_certificate_key("~/.config/connie/keys/key.pem");
//
fn main() {
        //env_logger::init();
        //let pki = TestPki::new();
        //let server_config = pki.server_config();
        let cert: Vec<CertificateDer> = load_cert("~/.config/connie/certificates/cert.pem").unwrap();
        let private_key: Vec<PrivateKeyDer> = load_private_certificate_key("~/.config/connie/keys/key.pem").unwrap();
        let private_key :PrivateKeyDer = private_key.iter().next().unwrap().clone_key();
        println!("private key is not a vec ");

        let mut server_config   = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(cert.clone(), private_key);
        let config = Arc::new(server_config.unwrap());
        //renaming it config back to server_config after Arc<.unwrap()>ing it;
        let server_config = config;

        let listener = std::net::TcpListener::bind(format!("192.168.7.13:{}", 443))
            .unwrap();
        for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                let mut acceptor = Acceptor::default();
                let accepted = loop {
                        acceptor.read_tls(&mut stream).unwrap();
                        if let Some(accepted) = acceptor.accept().unwrap(){
                                break accepted;
                        };
                };
                match accepted.into_connection(server_config.clone()){
                        Ok(mut conn ) => {
                         let msg = concat!(
                            "HTTP/1.1 200 OK\r\n",
                            "Connection: Closed\r\n",
                            "Content-Type: text/html\r\n",
                            "\r\n",
                            "<h1>Hello World!</h1>\r\n"
                         ).as_bytes();
                         // Note: do not use `unwrap()` on IO in real programs!
                        conn.writer().write_all(msg).unwrap();
                        conn.write_tls(&mut stream).unwrap();
                        conn.complete_io(&mut stream).unwrap();

                        conn.send_close_notify();
                        conn.write_tls(&mut stream).unwrap();
                        conn.complete_io(&mut stream).unwrap();
                    }
                    Err((err, _)) => {
                            eprintln!("{err}");
                    }
                }

        }
}

// struct TestPki {
//     server_cert_der: CertificateDer<'static>,
//     server_key_der: PrivateKeyDer<'static>,
// }

impl TestPki {
          fn mew() -> Self {
              let alg = &rcgen::PKCS_ECDSA_P256_SHA256;
              let mut ca_params = rcgen::CertificateParams::new(Vec::new()).unwrap();
              ca_params.subject_alt_names.push(rcgen::SanType::);
             ca_params.distinguished_name.push(rcgen::DnType::OrganizationalUnitName,"Connie");
             ca_params.distinguished_name.push(rcgen::DnType::CommonName, "connieserver");
          }
}

    fn load_cert(path: &str) -> Result<Vec<CertificateDer>, io::Error>  {//io::Result<Vec<CertificateDer>> {
        let cert_file = File::open(path).unwrap();
        let mut reader = BufReader::new(cert_file);
        let certs : Vec<CertificateDer> = rustls_pemfile::certs(&mut reader);

        Err(Error::new(ErrorKind::InvalidData, "Invalid Certificate")).expect("TODO: panic message");
        Ok(certs)
    }

    fn load_private_certificate_key(path: &str) -> io::Result<Vec<PrivateKeyDer>> {
        let private_key_file = File::open(path)?;
        let mut reader = BufReader::new(private_key_file);

        let keys : Vec<PrivateKeyDer> = rustls_pemfile::pkcs8_private_keys(&mut reader);

        Ok(keys)

    }




// #![warn(unused_variables)]
// //#![warn(unused_imports)]
// use std::io;
// use std::io::prelude::*;
// //use tokio::net::TcpListener;
// //use tokio::net::TcpStream;
// use std::net::TcpListener;
// use std::net::TcpStream;
//
// const STATE: u8 = 1;
//
// fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
//     //let state: u8 = 0;
//
//     let _ = stream.write(&STATE.to_be_bytes());
//     let _ = stream.flush();
//     let msg = stream.read(&mut [2])?;
//     println!("client: {}", &msg);
//     println!("passed to handle_connection");
//     Ok(())
// }
//
// fn main() {
//     let socket = TcpListener::bind("0.0.0.0:1909").expect("fail to bind");
//     println!("connected to port");
//     for stream in socket.incoming() {
//         match stream {
//             Ok(stream) => {
//                 let _ = handle_connection(stream);
//                 println!("a client connected");
//             }
//             Err(e) => {
//                 eprintln!("Error in match : {}", e);
//             }
//         }
//     }
// }

