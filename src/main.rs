use clap::{command, Arg, ArgMatches};
use lib_start::init::start;
use tokio::runtime::Runtime;

fn main() {
    let match_result: ArgMatches = command!()
        .arg(Arg::new("start").short('s').long("start"))
        .arg(Arg::new("bind").short('b').long("bind"))
        .get_matches();
    // make a PID lock filei in start

    let rt = Runtime::new().unwrap();

    let _machine = rt.block_on(start()).expect("could not get machine info");
}
