use std::fs::remove_file;
use tcp::system::process::process;

fn main() {
    let _ = process().unwrap();
    let lockfile = "/Connie/lockfile";
    let _ = remove_file(lockfile).unwrap();
}
