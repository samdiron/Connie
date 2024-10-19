//use clap;
use lib_start::init::start;
use tokio::runtime::Runtime;
fn main() {
    // make a PID lock file
    let rt = Runtime::new().unwrap();
    let machine =  rt.block_on(start()).expect("could not get machine info");
}
