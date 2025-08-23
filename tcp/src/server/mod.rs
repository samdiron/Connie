
pub mod listener;

pub(crate) mod req_format;
pub(crate) mod handle_client;

#[allow(dead_code)]
pub(crate) mod admin_requests;

pub(crate) mod serving_request;

pub(in crate::server) mod config;

/// ment to be used inside 1 threaded loop 
/// for runtime check and monitoring 
/// the system and keep statisics 
/// how many clients have connected and 
/// and unencrypt the data_dir
pub(in crate::server) mod runtime;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn get_server_tls_config() {
        
        let _config = config::make_config();

    }
    
}
