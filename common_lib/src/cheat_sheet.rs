#[test]
fn test_local_ip() {
    let ip = lip_fn();
    let stat_ip = LOCAL_IP.clone();
    debug_assert_eq!(stat_ip, ip);
}

use local_ip_addr::get_local_ip_address;
use std::net::IpAddr;
use std::process::exit;
use std::sync::LazyLock;

pub const POSTGRESQL_PORT: u16 = 5432;
pub const SYSTEM_TCP: u16 = 4445;
pub const MULTICAST_PORT: u16 = 4441;
pub const TCP_MAIN_PORT: u16 = 4443;
pub static LOCAL_IP: LazyLock<IpAddr> = LazyLock::new(|| lip_fn());

pub const DATA_DIR: &str = "/Connie/metadata";

pub const CRED: &str = "/Connie/etc/ident.conf";


fn lip_fn() -> IpAddr {
    let ip = get_local_ip_address();
    let ok: bool = ip.is_ok();
    if ok == false {
        exit(1111);
    }
    let ip = ip.unwrap();
    let ip = ip.parse::<IpAddr>().expect("ipaddr parse fail");
    ip
}
