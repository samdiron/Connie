#[test]
fn test_local_ip() {
    let ip = lip_fn();
    let stat_ip = LOCAL_IP.clone();
    debug_assert_eq!(stat_ip, ip);
}

use local_ip_addr::get_local_ip_address;
use std::process::exit;
use std::sync::LazyLock;

pub const MULTICAST_PORT: u16 = 4441;
pub const TCP_MAIN_PORT: u16 = 4443;
pub static LOCAL_IP: LazyLock<String> = LazyLock::new(|| lip_fn());

fn lip_fn() -> String {
    let ip = get_local_ip_address();
    let ok: bool = ip.is_ok();
    if ok == false {
        exit(1111);
    }
    let ip = ip.unwrap();
    ip
}
