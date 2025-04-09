pub mod server;
pub mod client;
pub(crate) mod common;

#[allow(unused_imports)]
pub mod types {
    use crate::common::request;
    pub use request::RQM;
    pub use request::{
        GET,
        POST,
        DELETE,
        JWT_AUTH,
        LOGIN_CRED,
        
    };
}


pub mod consts {
    use std::sync::Mutex;
    pub const NET_STATUS:u8 = 1;
    pub const PRIVATE_STATUS: u8 = 0;
    
    pub static NEW_USERS: Mutex<u8> = Mutex::new(0);

    pub static PORT: Mutex<u16> = Mutex::new(0);


    pub static USE_IP: Mutex<u8> = Mutex::new(PRIVATE_STATUS);

    pub static USE_PORT: Mutex<u8> = Mutex::new(0);

}
