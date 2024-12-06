// use tokio;
use std::net::{IpAddr, SocketAddr,TcpListener};
use common_lib::cheat_sheet::{LOCAL_IP,TCP_MAIN_PORT};

use std::io::Write;
use std::sync::Arc;

use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use rustls::server::Acceptor;
use rustls::ServerConfig;


pub fn tcp_listener() {
    let ip: IpAddr = LOCAL_IP.clone();
    let port: u16 = TCP_MAIN_PORT.clone();
    let pki = TestPki::new();
    let config = pki.server_config();
    let socket_addr = SocketAddr::new(ip, port);
    let listener = TcpListener::bind(socket_addr)
        .expect("could not bind tcp socket on port 4443 ");
    println!("tcp socket open on port 4443");
    for stream in listener.accept() {
        let stream_addr = stream.1;
        let mut stream  = stream.0;

        let mut acceptor = Acceptor::default();
        
        let accepted = loop {
            acceptor.read_tls(&mut stream).unwrap();
            if let Some(accepted) = acceptor.accept().unwrap(){
                break accepted;
            }
        };
        match accepted.into_connection(config.clone()) {
            Ok(mut conn) => {
                let info_msg = stream_addr.to_string();
                let hello_msg = format!("hello {info_msg}");
                //TODO: error msgs for tcp listener
                conn.writer().write_all(hello_msg.as_bytes()).expect("todo error msg stream write");
                conn.write_tls(&mut stream).expect("todo error msg s writetls");
                conn.complete_io(&mut stream).expect("error complete io");
                println!("ok");

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
        let ip_addr = LOCAL_IP.clone();
        let alg = &rcgen::PKCS_ECDSA_P256_SHA256;
        let mut ca_params = rcgen::CertificateParams::new(Vec::new())
            .unwrap();
        ca_params
            .subject_alt_names
            .push(rcgen::SanType::IpAddress(ip_addr));
        ca_params
            .distinguished_name
            .push(rcgen::DnType::CommonName, "Connie Server");
        ca_params
            .is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
        ca_params.key_usages = vec![
            rcgen::KeyUsagePurpose::KeyCertSign,
            rcgen::KeyUsagePurpose::DigitalSignature,
        ];
        let ca_key = rcgen::KeyPair::generate_for(alg)
            .unwrap();
        let ca_cert = ca_params.self_signed(&ca_key)
            .unwrap();

        //server 
        let mut server_e_params = rcgen::CertificateParams::new(vec!["connie".to_string()])
            .unwrap();
        server_e_params.is_ca = rcgen::IsCa::NoCa;
        server_e_params.extended_key_usages = vec![rcgen::ExtendedKeyUsagePurpose::ServerAuth];
        let server_key = rcgen::KeyPair::generate_for(alg)
            .unwrap();
        let server_cert = server_e_params
            .signed_by(&server_key, &ca_cert, &ca_key)
            .unwrap();
        Self {
            server_key_der: PrivatePkcs8KeyDer::from(server_key.serialize_der()).into(),
            server_cert_der: server_cert.into()
        }

    }

    fn server_config(self) -> Arc<ServerConfig> {
        let mut server_config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(
                vec![self.server_cert_der],
                self.server_key_der
            ).unwrap();
        server_config.key_log = Arc::new(rustls::KeyLogFile::new());
        Arc::new(server_config)
    }
}











