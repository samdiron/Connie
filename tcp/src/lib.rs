pub mod server;
pub mod client;
pub(crate) mod common;

pub mod consts {
    use std::{net::IpAddr, sync::Mutex};

    pub static IP: Mutex<&str> = Mutex::new("");

    pub static PORT: Mutex<u16> = Mutex::new(0);


    pub static USE_IP: Mutex<u8> = Mutex::new(0);

    pub static USE_PORT: Mutex<u8> = Mutex::new(0);



}
