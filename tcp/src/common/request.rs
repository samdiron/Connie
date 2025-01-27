#[allow(dead_code)]
pub const JWT_AUTH: &str = "0";
#[allow(dead_code)]
pub const LOGIN_CRED: &str = "1";
#[allow(dead_code)]
pub const SIGNIN_CRED: &str = "2";

pub const PACKET_SIZE: u16 = 65535;

//request Head

#[allow(dead_code)]
pub const GET: &str = "!G: ";
#[allow(dead_code)]
pub const POST: &str = "!P: ";
#[allow(dead_code)]
pub const DELETE: &str = "!D: ";
#[allow(dead_code)]
pub const REQUEST_HEAD: &str = "!rq: ";

#[allow(dead_code)]
pub const SPLIT: &str = "\n";
#[allow(dead_code)]
pub const END: &str = "\0";

#[allow(dead_code)]
pub const JWT_HEAD: &str = "JWT: ";

#[allow(dead_code)]
pub const INVALID_RQ: &str = "INVALID";

#[allow(dead_code)]
pub const PASSWORD_HEADER: &str = "PassWd: ";
#[allow(dead_code)]
pub const CPID_HEADER: &str = "CpId: ";
#[allow(dead_code)]
pub fn format_jwt_request(jwt: String, request: String) -> String {
    let string = format!("{JWT_AUTH}{SPLIT}{JWT_HEAD}{jwt}{SPLIT}{REQUEST_HEAD}{request}{END}");
    string
}

#[allow(dead_code)]
pub fn format_login_request(cred: Vec<&str>) -> String {
    let cpid = cred[0];
    let password = cred[1];
    let string =
        format!("{LOGIN_CRED}{SPLIT}{CPID_HEADER}{cpid}{SPLIT}{PASSWORD_HEADER}{password}{END}");
    string
}

#[allow(dead_code)]
pub fn split_request(request: String) -> Vec<String> {
    let split_arr: Vec<&str> = request.split(SPLIT).collect();
    let mut local_request: Vec<String> = Vec::new();
    match split_arr[0] {
        JWT_HEAD => {
            //adding the jwt
            local_request.push(JWT_HEAD.to_string());
            let jwt_arr: Vec<&str> = split_arr[1].split(JWT_HEAD).collect();
            let jwt = jwt_arr[0];
            local_request.push(jwt.to_string());

            //adding the request
            local_request.push(split_arr[2].to_string());
        }
        LOGIN_CRED => {
            local_request.push(LOGIN_CRED.to_string());
            local_request.push(split_arr[1].to_string());
            local_request.push(split_arr[2].to_string());
        }
        SIGNIN_CRED => {
            local_request.push(SIGNIN_CRED.to_string());
        }
        _ => {
            local_request.insert(0, INVALID_RQ.to_string());
        }
    }
    local_request
}





