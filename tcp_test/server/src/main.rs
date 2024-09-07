use std::io::{Error, ErrorKind, Write};
use std::sync::Arc;
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use rustls::server::{Acceptor, NoClientAuth};
use rustls::{ConfigBuilder, ServerConfig};
//use std::io::{self, BufReader};
use std::fs::File;
use std::net::IpAddr;
use rustls::crypto::CryptoProvider;
use rcgen::{Certificate, CertificateParams, PKCS_ECDSA_P256_SHA256, SanType};
use local_ip_address::local_ip;
pub static LIP: IpAddr = local_ip().unwrap();
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

struct TestPki {
     server_cert_der: CertificateDer<'static>,
     server_key_der: PrivateKeyDer<'static>,
}


impl TestPki {
    fn new() -> Self {
        let alg = &rcgen::PKCS_ECDSA_P256_SHA256;
        let mut ca_params = rcgen::CertificateParams::new(Vec::new()).unwrap();
        ca_params.subject_alt_names.push(rcgen::SanType::IpAddress(LIP));
        ca_params
            .distinguished_name
            .push(rcgen::DnType::OrganizationName, "Provider Server Example");
        ca_params
            .distinguished_name
            .push(rcgen::DnType::CommonName, "Example CA");
        ca_params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
        ca_params.key_usages = vec![
            rcgen::KeyUsagePurpose::KeyCertSign,
            rcgen::KeyUsagePurpose::DigitalSignature,
        ];
        let ca_key = rcgen::KeyPair::generate_for(alg).unwrap();
        let ca_cert = ca_params.self_signed(&ca_key).unwrap();

        // Create a server end entity cert issued by the CA.
        let mut server_ee_params =
            rcgen::CertificateParams::new(vec!["localhost".to_string()]).unwrap();
        server_ee_params.is_ca = rcgen::IsCa::NoCa;
        server_ee_params.extended_key_usages = vec![rcgen::ExtendedKeyUsagePurpose::ServerAuth];
        let server_key = rcgen::KeyPair::generate_for(alg).unwrap();
        let server_cert = server_ee_params
            .signed_by(&server_key, &ca_cert, &ca_key)
            .unwrap();
        Self {
            server_cert_der: server_cert.into(),
            // TODO(XXX): update below once https://github.com/rustls/rcgen/issues/260 is resolved.
            server_key_der: PrivatePkcs8KeyDer::from(server_key.serialize_der()).into(),
        }
    }

    fn server_config(self) -> Arc<ServerConfig> {
        let mut server_config =
            ServerConfig::builder_with_details()
                .with_safe_default_protocol_versions()
                .unwrap()
                .with_no_client_auth()
                .with_single_cert(vec![self.server_cert_der], self.server_key_der)
                .unwrap();

        server_config.key_log = Arc::new(rustls::KeyLogFile::new());

        Arc::new(server_config)
    }
}



// impl TestPki {
//           fn mew() -> Self {
//               let alg = &rcgen::PKCS_ECDSA_P256_SHA256;
//               let mut ca_params = rcgen::CertificateParams::new(Vec::new()).unwrap();
//               ca_params.subject_alt_names.push(rcgen::SanType::IpAddress(LIP.clone()));
//               //ca_params.distinguished_name.push(rcgen::DnType::OrganizationalUnitName,"Connie");
//               //ca_params.distinguished_name.push(rcgen::DnType::CommonName, "connieserver");
//           }
// }
//
//


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

