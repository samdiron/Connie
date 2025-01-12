use rustls::ServerConfig;
use rustls::pki_types::PrivatePkcs8KeyDer;
use std::sync::Arc;
use rustls;
use common_lib::cheat_sheet::LOCAL_IP;
use rcgen;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};


pub struct TestPki {
    server_cert_der: CertificateDer<'static>,
    server_key_der: PrivateKeyDer<'static>,
}

impl TestPki {
    pub fn new() -> Self {
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

    pub fn server_config(self) -> Arc<ServerConfig> {
        let mut server_config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(vec![self.server_cert_der], self.server_key_der)
            .unwrap();
        server_config.key_log = Arc::new(rustls::KeyLogFile::new());
        Arc::new(server_config)
    }
}
