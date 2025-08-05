
pub mod listener;

pub(crate) mod req_format;
pub(crate) mod handle_client;
pub(crate) mod serving_request;

pub(in crate::server) mod config;
// pub(in crate::server) mod admin_requests;

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
