use common_lib::{log::warn, path::LOGS_PATH};
use lib_db::fncs::random_string;


use std::{fs::File, io::Write, time::SystemTime};


pub struct ClientErrorMsgLog {
    pub client_jwt_cpid: String,
    pub client_request_cpid: String,
    pub client_ip: String,
    pub timestamp: SystemTime,
    pub sev: u8
}



pub fn client_cpid_not_match(
  s: &ClientErrorMsgLog  
)-> String {
    let error_msg = format!(r#"
    a client sent a bad request that had 2 different IDs 
    in the request and the JWT token 
    this activity is suspicious 
    all request for the next 3 hours from this client will be denyed 
    JWT CPID: {}
    request CPID: {}
    Ip of said request: {}
    system_time: {:?} "#,
        s.client_jwt_cpid,
        s.client_request_cpid,
        s.client_ip,
        s.timestamp,
);

    let log_filename = random_string(19);
    let log_file_path = format!("{LOGS_PATH}/{log_filename}.log");
    let f = File::create_new(&log_file_path);
    if f.is_err() {
        warn!("unable to create a log file in {LOGS_PATH}");
    }else {
        let mut file = f.unwrap();
        file.write_all(error_msg.as_bytes()).unwrap();
    }
    return log_file_path

}
