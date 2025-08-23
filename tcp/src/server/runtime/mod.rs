
#![allow(dead_code)]
/// constains the functions and template for reporting any suspicious user activity 
/// and stores logs in CLIENT_LOG_F
pub mod logs;
/// constains the static vars that the listener uses to control the flow 
/// of requests && keep track of  bandwidth and other metrics 
/// to show on the admin dashboard
pub mod statics;
/// contains helper function for counting files in storage and keeping db records up to date
pub mod file_checks;
/// constains all the functions of the public_files process 
pub mod public_files;
/// contains helper functions to count and sort requests for admin dashboard
pub mod sort_requests;
/// creating a stand alone file with a detailed report
pub mod generate_log_templates;




