#![allow(dead_code)]
use std::{
    sync::Arc,
    path::{Path, PathBuf},
};

use common_lib::path::{CERIFICATE_PATH, PRIVATEKEY_PATH};

use tokio_rustls::rustls::pki_types::{
    pem::PemObject,
    CertificateDer,
    PrivateKeyDer
};
use tokio_rustls::rustls::{
    self,
    KeyLogFile,
    ServerConfig,
    DEFAULT_VERSIONS,
    crypto::CryptoProvider,
};

use rustls::crypto::aws_lc_rs as provider;

fn load_certs(filename: &Path) -> Vec<CertificateDer<'static>> {
    CertificateDer::pem_file_iter(filename)
        .expect("cannot open certificate file")
        .map(|result| result.unwrap())
        .collect()
}

fn load_private_key(filename: &Path) -> PrivateKeyDer<'static> {
    PrivateKeyDer::from_pem_file(filename)
        .expect("cannot read private key file")
}


pub fn make_config() -> Arc<ServerConfig> {
    let cpath = PathBuf::from(CERIFICATE_PATH);
    let kpath = PathBuf::from(PRIVATEKEY_PATH);
    let cert = load_certs(&cpath);
    let key = load_private_key(&kpath);
    let suites = provider::ALL_CIPHER_SUITES.to_vec();
    let version = DEFAULT_VERSIONS;
    let mut config = ServerConfig::builder_with_provider(
        CryptoProvider {
            cipher_suites: suites,
            ..provider::default_provider()
        }.into()
    ).with_protocol_versions(&version)
    .expect("could not create config")
    .with_no_client_auth()
    .with_single_cert(cert, key)
    .expect("could not create server config 2");

    config.key_log = Arc::new(KeyLogFile::new());
    config.ticketer = provider::Ticketer::new()
        .expect("faild to get server tls ticketer");

    Arc::new(config)



    
}
