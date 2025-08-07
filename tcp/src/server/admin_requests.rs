#![allow(non_camel_case_types)]

use std::time::Duration;

type Obool = Option<bool>;  

use lib_db::{
    media::media::Media,
    user::user_struct::User
};


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


