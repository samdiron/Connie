pub mod server;
pub mod client;
pub(crate) mod common;

#[allow(unused_imports)]
pub mod types {
    use crate::common::request;
    pub use request::RQM;
    pub use request::{
        JWT_AUTH,
        LOGIN_CRED,
        GET,
        POST,
        DELETE,
        
    };
}


pub mod consts {
    use std::sync::Mutex;

    pub static IP: Mutex<&str> = Mutex::new("");

    pub static PORT: Mutex<u16> = Mutex::new(0);


    pub static USE_IP: Mutex<u8> = Mutex::new(0);

    pub static USE_PORT: Mutex<u8> = Mutex::new(0);



}
