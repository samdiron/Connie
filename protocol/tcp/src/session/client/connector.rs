// use std::io::{Read, Stdout, Write};
use std::io::Write;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::sync::Arc;
use tokio_rustls::rustls;

pub fn tcp_connector() {
    let root_store =
        rustls::RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    let ip = IpAddr::from_str("ip_from the multicast search or a known server").unwrap();
    let port = common_lib::cheat_sheet::TCP_MAIN_PORT;
    let socketaddr = SocketAddr::new(ip, port);
    let server_name = "connie".try_into().unwrap();
    let mut conn = rustls::ClientConnection::new(Arc::new(config), server_name).unwrap();
    let mut sock = TcpStream::connect(socketaddr).unwrap();
    let mut tls = rustls::Stream::new(&mut conn, &mut sock);
    let text = b"text from a client ?.";
    let _res = tls.write_all(text).unwrap();
    let ciphersuite = tls.conn.negotiated_cipher_suite().unwrap();

    writeln!(
        &mut std::io::stderr(),
        "current suite: {:?}",
        ciphersuite.suite()
    )
    .unwrap();
}
