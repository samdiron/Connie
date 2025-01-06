use crate::common::request::{JWT_HEAD, SPLIT};

use common_lib::cheat_sheet::{LOCAL_IP, TCP_MAIN_PORT};

use lib_db::database::get_conn;
use lib_db::jwt;
use lib_db::types::{PgPool, Result};
use lib_db::user::user_struct::vaildate_claim;

use log::info;

use std::io::Read as reader;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

use std::net::{TcpListener, TcpStream};

use rustls;
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use rustls::server::Acceptor;
use rustls::{ServerConfig, ServerConnection};

//NOTE: use mio instead of std::net

// will wait untile the new db is written
//
// hansshake buffer example:
// 0\1\2\3
// 0 means a jwt after the next \n
// 1 means no jwt but a known user struct after the next \n
// 2 means no jwt but a unknown user struct after the next \n note will prompt a password
// 3 means a server will o the logic latter for it
//
// then the request PUSH or GET:FILE:filename ;
// or an edit request EDIT FILE:old_name,new_name
// or a fetch request FETCH user files group all like recenta

const READ_ERROR: &str = "ERROR could not read a buffer IO/TLS/complete_io ";

async fn process_request(raw_text: &str, pool: &PgPool) -> Result<()> {
    let buffer: Vec<&str> = raw_text.split(SPLIT).collect();
    let mut valid_auth: bool = false;
    match buffer[0] {
        "0" => {
            let jwt = buffer[1].trim_start_matches(JWT_HEAD).to_string();
            let claim = jwt::validate(&jwt).await?;
            if vaildate_claim(claim.cpid, claim.paswd, &pool).await.is_ok() {
                *&mut valid_auth = true;
            }
        }

        _ => {
            println!("a invalid request ");
        }
    }
    Ok(())
}

async fn handle(
    mut conn: ServerConnection,
    mut stream: TcpStream,
    sock_addr: SocketAddr,
    pool: &PgPool,
) -> std::io::Result<()> {
    let _is_handshake = conn.process_new_packets().unwrap();
    let mut string_buff = String::new();
    // let mut buffer = vec![0; 150];

    if conn.wants_read() {
        conn.reader()
            .read_to_string(&mut string_buff)
            .expect(READ_ERROR);
        conn.read_tls(&mut stream)?;
        conn.complete_io(&mut stream)?;
        let _ = process_request(string_buff.as_str(), &pool);
    }
    Ok(())
}

pub async fn tcp_listener() {
    let pool = get_conn().await.unwrap();
    let ip: IpAddr = LOCAL_IP.clone();
    let port: u16 = TCP_MAIN_PORT.clone();

    let pki = TestPki::new();
    let config = pki.server_config();

    let socket_addr = SocketAddr::new(ip, port);

    let listener = TcpListener::bind(socket_addr).expect("could not bind tcp socket on port 4443 ");

    println!("tcp socket open on port: {}", TCP_MAIN_PORT);

    for stream in listener.accept() {
        let sock_addr = stream.1;
        let stream = stream.0;

        let mut acceptor = Acceptor::default();

        let accepted = loop {
            if let Some(accepted) = acceptor.accept().unwrap() {
                break accepted;
            }
        };

        match accepted.into_connection(config.clone()) {
            Ok(conn) => {
                info!("conn will be handled ip: {} ;", sock_addr.ip().clone());

                handle(conn, stream, sock_addr, &pool).await;
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
