
pub fn display_size(size: u64) -> String {

    let string_size: String;
    if size < 1000 {
        string_size = format!("{size}Bytes");
        return string_size;
    }
    let fkb = size as f64 / 1000.00 ;
    if fkb < 1000.00 {
        string_size = format!("{:.2}KB", fkb);
        return string_size
    };
    let fmb = fkb / 1000.00;
    if fmb < 1000.00 {
        string_size = format!("{:.2}MB", fmb);
        return string_size
    };
    let fgb = fmb / 1000.00;
    if fgb < 1000.00 {
        string_size = format!("{:.2}GB", fgb);
        return string_size
    }
    let fpb = fgb / 1000.00;
    return format!("{:.2}PB", fpb);
}



pub mod cheat_sheet;
pub mod path;


pub use log;

pub use rand;
pub use toml;

pub use tokio;
pub use serde;

pub use sha256;

pub use bincode;
pub use sysinfo;

pub use public_ip;
pub use rpassword;

pub use gethostname;
