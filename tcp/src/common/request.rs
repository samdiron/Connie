
use log::warn;

// AUTH
pub const JWT_AUTH: &str = "0";
pub const LOGIN_CRED: &str = "1";
pub const SIGNIN_CRED: &str = "2";

//request Head
pub const GET: &str = "!G: ";
pub const POST: &str = "!P: ";
pub const DELETE: &str = "!D: ";
pub const REQUEST_HEAD: &str = "!rq: ";

pub const SPLIT: &str = "\n";
pub const END: &str = "\0";

pub const JWT_HEAD: &str = "JWT: ";

pub const INVALID_RQ: &str = "INVALID";

pub const PASSWORD_HEADER: &str = "PassWd: ";
pub const CPID_HEADER: &str = "CpId: ";
pub fn format_jwt_request(jwt: String, request: String) -> String {
    let string = format!("{JWT_AUTH}{SPLIT}{JWT_HEAD}{jwt}{SPLIT}{REQUEST_HEAD}{request}{END}");
    string
}

pub fn format_login_request(cred: Vec<&str>) -> String {
    let cpid = cred[0];
    let password = cred[1];
