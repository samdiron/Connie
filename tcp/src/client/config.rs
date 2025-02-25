use std::sync::Arc;

use tokio_rustls::rustls::{
    self,
    crypto::{
        aws_lc_rs as provider,
        ring::DEFAULT_CIPHER_SUITES,
        CryptoProvider
    },
    ClientConfig,
    RootCertStore,
    DEFAULT_VERSIONS
};


mod danger {
    use tokio_rustls::rustls as rustls;
    use rustls::DigitallySignedStruct;
    use rustls::client::danger::HandshakeSignatureValid;
    use rustls::crypto::{CryptoProvider, verify_tls12_signature, verify_tls13_signature};
    use rustls::pki_types::{CertificateDer, ServerName, UnixTime};

    #[derive(Debug)]
    pub struct NoCertificateVerification(CryptoProvider);

    impl NoCertificateVerification {
        pub fn new(provider: CryptoProvider) -> Self {
            Self(provider)
        }
    }

    impl rustls::client::danger::ServerCertVerifier for NoCertificateVerification {
        fn verify_server_cert(
            &self,
            _end_entity: &CertificateDer<'_>,
            _intermediates: &[CertificateDer<'_>],
            _server_name: &ServerName<'_>,
            _ocsp: &[u8],
            _now: UnixTime,
        ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
            Ok(rustls::client::danger::ServerCertVerified::assertion())
        }

        fn verify_tls12_signature(
            &self,
            message: &[u8],
            cert: &CertificateDer<'_>,
            dss: &DigitallySignedStruct,
        ) -> Result<HandshakeSignatureValid, rustls::Error> {
            verify_tls12_signature(
                message,
                cert,
                dss,
                &self.0.signature_verification_algorithms,
            )
        }

        fn verify_tls13_signature(
            &self,
            message: &[u8],
            cert: &CertificateDer<'_>,
            dss: &DigitallySignedStruct,
        ) -> Result<HandshakeSignatureValid, rustls::Error> {
            verify_tls13_signature(
                message,
                cert,
                dss,
                &self.0.signature_verification_algorithms,
            )
        }

        fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
            self.0
                .signature_verification_algorithms
                .supported_schemes()
        }
    }
}

#[allow(dead_code)]
pub async fn make_config() -> Arc<rustls::ClientConfig> {
    let mut root_store = RootCertStore::empty();
    root_store.extend(
        webpki_roots::TLS_SERVER_ROOTS
            .iter()
            .cloned()
    );

    let suites = DEFAULT_CIPHER_SUITES.to_vec();
    let version = DEFAULT_VERSIONS;

    let mut config = ClientConfig::builder_with_provider(CryptoProvider {
        cipher_suites: suites,
        ..provider::default_provider()
    }.into()
    ).with_protocol_versions(&version)
    .expect("could not make client tls config")
    .with_root_certificates(root_store)
    .with_no_client_auth();

    config.key_log = Arc::new(rustls::KeyLogFile::new());
    config.dangerous()
            .set_certificate_verifier(Arc::new(danger::NoCertificateVerification::new(
                provider::default_provider(),
            )));

    Arc::new(config)

}



