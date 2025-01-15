use crate::common::request::{JWT_HEAD, SPLIT};
use log::info;

use common_lib::cheat_sheet::{LOCAL_IP, TCP_MAIN_PORT};

use lib_db::database::get_conn;
use lib_db::jwt;
use lib_db::types::{PgPool, Result};
use lib_db::user::user_struct::vaildate_claim;

use std::net::{IpAddr, SocketAddr};

use std::net::{TcpListener, TcpStream};

//NOTE: use mio instead of std::net

// will wait untile the new db is written
//
// hansshake buffer example:
// 0\1\2\3
// 0 means a jwt after the next \n
// 1 means no jwt but a known user struct after the next \n
// 2 means no jwt but a unknown user struct after the next \n note will prompt a password
// 3 means a server will o the logic latter for it
//
// then the request PUSH or GET:FILE:filename ;
// or an edit request EDIT FILE:old_name,new_name
// or a fetch request FETCH user files group all like recenta

const READ_ERROR: &str = "ERROR could not read a buffer IO/TLS/complete_io ";

async fn process_request(raw_text: &str, pool: &PgPool) -> Result<()> {
    let buffer: Vec<&str> = raw_text.split(SPLIT).collect();
    let mut valid_auth: bool = false;
    match buffer[0] {
        "0" => {
            let jwt = buffer[1].trim_start_matches(JWT_HEAD).to_string();
            let claim = jwt::validate(&jwt).await?;
            if vaildate_claim(claim.cpid, claim.paswd, &pool).await.is_ok() {
                *&mut valid_auth = true;
            }
        }

        _ => {
            println!("a invalid request ");
        }
    }
    Ok(())
}

async fn handle(
    mut stream: TcpStream,
    sock_addr: SocketAddr,
    pool: &PgPool,
) -> std::io::Result<()> {
    let ip = format!("{sock_addr}");
    info!("handling {ip}");

    Ok(())
}

pub async fn tcp_listener() {
    let workers = std::thread::available_parallelism().unwrap();

    let pool = get_conn().await.unwrap();
    let ip: IpAddr = LOCAL_IP.clone();
    let port: u16 = TCP_MAIN_PORT.clone();

    let socket_addr = SocketAddr::new(ip, port);

    let listener = TcpListener::bind(socket_addr).expect("could not bind tcp socket on port 4443 ");

    println!("tcp socket open on port: {}", TCP_MAIN_PORT);
    for stream in listener.accept() {
        let sock_addr = stream.1;
        let stream = stream.0;
        info!("{ip} connected");

        handle(stream, sock_addr, &pool);
    }
}
