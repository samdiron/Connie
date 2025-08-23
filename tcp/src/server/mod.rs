pub mod listener;

pub(crate) mod req_format;

pub(in crate::server) mod config;

pub(in crate::server) mod request_handles;

pub(in crate::server) mod runtime;


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn get_server_tls_config() {
        
        let _config = config::make_config();

    }
    
}
