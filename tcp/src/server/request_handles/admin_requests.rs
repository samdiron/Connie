#![allow(non_camel_case_types)]

use std::{
    io,
    time::Duration
};

type Obool = Option<bool>;  

use common_lib as cl;
use cl::bincode;
use lib_db::{
    media::media::Media, types::PgPool, user::user_struct::User
};
use serde::{Deserialize, Serialize};
use tokio::{io::AsyncReadExt, net::TcpStream};
use tokio_rustls::TlsStream;

// #[derive(Clone, Deserialize, Serialize)]
struct STATS {
    pid: String,
    uptime: Duration,
    n_get_requests: u64,
    no_tls_status: bool,
    storage_usage: usize,
    network_usage: usize,
    n_post_requests: u64,
    failed_requests: u64,
    allow_new_users: bool,
    n_current_requests: u64,
    invalid_tls_reqests: u64,
    successful_requests: u64,
    list_current_users: Vec<User>,
    list_all_files_in_storage: Vec<Media>,
}



#[derive(Clone, Deserialize, Serialize)]
pub enum ADMINREQS {
    STATS {
        all: Obool,
        pid: Obool,
        uptime: Obool,
        no_tls_status: Obool,
        storage_usage: Obool,
        network_usage: Obool,
        n_get_requests: Obool,
        list_all_files: Obool,
        n_post_requests: Obool,
        failed_requests: Obool,
        allow_new_users: Obool,
        list_current_users: Obool,
        n_current_requests: Obool,
        invalid_tls_reqests: Obool,
        successful_requests: Obool,
    },

    SERVER {
        no_tls: Obool,
        unpause: Obool,
        restart: Obool,
        new_users: Obool,
        soft_pause: Obool,
        hard_pause: Obool,
        refresh_pub_files: Obool,
    },


}


impl ADMINREQS {

    fn dz(buf: Vec<u8>) -> Result<Self, bincode::Error> {
        let res: Self = bincode::deserialize(&buf)?;
        drop(buf);
        Ok(res)
    }
}


// blue prints to the server admin function
async fn handle_admin(
    mut stream: TlsStream<TcpStream>,  
    pool: &PgPool,
) -> io::Result<()> {
    let request_size = stream.read_u32().await?;
    let mut request_buf= vec![0;request_size as usize];
    stream.read_exact(&mut request_buf).await?;
    let request = ADMINREQS::dz(request_buf);
    if request.is_err() {
        // warn and freak out and create logs
    };
    // unwrap and then match the request and change static values in the main loop 
    // to control the flow of users
    // and count and analyze the requests 
    Ok(())
}










