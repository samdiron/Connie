// use tokio;
use common_lib::cheat_sheet::{LOCAL_IP, TCP_MAIN_PORT};
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};

use std::io::{Read, Write};
use std::sync::Arc;

use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use rustls::server::Acceptor;
use rustls::{Connection, ServerConfig, ServerConnection};
//
// hansshake buffer example:
// 0\1\2\3
// 0 means a jwt after the next \n
// 1 means no jwt but a known user struct after the next \n
// 2 means no jwt but a unkown user struct after the next \n note will prompt a password
// 3 means a server will o the logic latter for it
//
// then the request PUSH or GET:FILE:filename ;
// or an edit request EDIT FILE:old_name,new_name
// or a fetch request FETCH user files group all like recent

const READ_ERROR: &str = "ERROR could not read a buffer IO/TLS/complete_io ";

fn process_request(buffer: &Vec<&str>) {
    match buffer[0] {
        "0" => {
            buffer[1];
        }
        _ => {
            println!("a invalid request ")
        }
    }
}

fn handle(mut conn: ServerConnection, mut stream: TcpStream) {
    let mut conn = conn;
    let mut stream = stream;
    let mut _is_handshake = conn.process_new_packets().unwrap();
    let mut string_buff = String::new();
    let mut buffer = vec![0; 150];

    if conn.wants_read() {
        conn.reader()
            .read_to_string(&mut string_buff)
            .expect(READ_ERROR);
        conn.read_tls(&mut stream).expect(READ_ERROR);
        conn.complete_io(&mut stream).expect(READ_ERROR);
        let message_vec: Vec<&str> = string_buff.split("\n").collect();
    }
}

pub fn tcp_listener() {
    let ip: IpAddr = LOCAL_IP.clone();
    let port: u16 = TCP_MAIN_PORT.clone();
    let pki = TestPki::new();
    let config = pki.server_config();
    let socket_addr = SocketAddr::new(ip, port);
    let listener = TcpListener::bind(socket_addr).expect("could not bind tcp socket on port 4443 ");
    println!("tcp socket open on port: {}", TCP_MAIN_PORT);
    for stream in listener.accept() {
        let stream_addr = stream.1;
        let mut stream = stream.0;

        let mut acceptor = Acceptor::default();

        let accepted = loop {
            acceptor.read_tls(&mut stream).unwrap();
            if let Some(accepted) = acceptor.accept().unwrap() {
                break accepted;
            }
        };
        match accepted.into_connection(config.clone()) {
            Ok(mut conn) => {

                // let info_msg = stream_addr.to_string();
                // let hello_msg = format!("hello {info_msg}");
                // //TODO: error msgs for tcp listener
                // conn.writer()
                //     .write_all(hello_msg.as_bytes())
                //     .expect("todo error msg stream write");
                // conn.write_tls(&mut stream)
                //     .expect("todo error msg s writetls");
                // conn.complete_io(&mut stream).expect("error complete io");
                // //TODO: auth , process the request , notify user for ending the conn
                // println!("ok");
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
        let mut ca_params = rcgen::CertificateParams::new(Vec::new()).unwrap();
        ca_params
            .subject_alt_names
            .push(rcgen::SanType::IpAddress(ip_addr));
        ca_params
            .distinguished_name
            .push(rcgen::DnType::CommonName, "Connie Server");
        ca_params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
        ca_params.key_usages = vec![
            rcgen::KeyUsagePurpose::KeyCertSign,
            rcgen::KeyUsagePurpose::DigitalSignature,
        ];
        let ca_key = rcgen::KeyPair::generate_for(alg).unwrap();
        let ca_cert = ca_params.self_signed(&ca_key).unwrap();

        //server
        let mut server_e_params =
            rcgen::CertificateParams::new(vec!["connie".to_string()]).unwrap();
        server_e_params.is_ca = rcgen::IsCa::NoCa;
        server_e_params.extended_key_usages = vec![rcgen::ExtendedKeyUsagePurpose::ServerAuth];
        let server_key = rcgen::KeyPair::generate_for(alg).unwrap();
        let server_cert = server_e_params
            .signed_by(&server_key, &ca_cert, &ca_key)
            .unwrap();
        Self {
            server_key_der: PrivatePkcs8KeyDer::from(server_key.serialize_der()).into(),
            server_cert_der: server_cert.into(),
        }
    }

    fn server_config(self) -> Arc<ServerConfig> {
        let mut server_config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(vec![self.server_cert_der], self.server_key_der)
            .unwrap();
        server_config.key_log = Arc::new(rustls::KeyLogFile::new());
        Arc::new(server_config)
    }
}
