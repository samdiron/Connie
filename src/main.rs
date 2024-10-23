// use std::process::exit;
use clap::{command, ArgGroup, Arg, ArgMatches};
use lib_start::init::start;
use tokio::runtime::Runtime;
//args start connect bind verbose sql max-clients only-users web-host  
fn main() {
    let match_result: ArgMatches = command!()
        .about("Connie is a private home server used to connect with other connie servers privately it can stream movies music and store private files ")
        .group(ArgGroup::new("start-server"))
        .arg(Arg::new("start")
            .short('s')
            .long("start")
            .num_args(0)
            .help("ready to connect/bind mode"))
        .group(ArgGroup::new("server-user"))
        .arg(Arg::new("bind")
            .short('b')
            .long("bind")
            .conflicts_with("connect").requires("start")
            .help("binds connie to local_ip:4443 tcp"))
        .arg(Arg::new("connect")
            .long("connect").requires("start"))
        .arg(Arg::new("verbose").long("verbose").requires("start"))// .arg(Arg::new("bind")
        //     .short('b')
        //     .long("bind")
        //     .conflicts_with("connect")
        //     .help("binds connie to local_ip:4443 tcp"))
        // .arg(Arg::new("connect")
        //     .short("c")
        //     .long("connect"))
        // .arg(Arg::new("verbose").short("v").long("verbose"))    
        .get_matches();
    
    // make a PID lock file in start
     let rt = Runtime::new().unwrap();

    let is_ready : bool =  match_result.get_one::<bool>("start").unwrap().clone();

    println!("arg passed");
    if is_ready {

        let machine = rt.block_on(start()).expect("could not get machine info");
        println!("{}",machine.host_name);
        println!("connie is up and ready");
        let matches = rt.block_on(commands()).expect("could not get user input").clone();
        let matches_ = matches.get_one::<bool>("connect").unwrap().clone();
        if matches_ {
        println!("okay");
        }
    }
}


async fn commands() -> std::io::Result<ArgMatches> {
let match_result: ArgMatches = command!()
        .about("Connie is a private home server used to connect with other connie servers privately it can stream movies music and store private files ")
        .group(ArgGroup::new("server-user"))
       .arg(Arg::new("bind")
            .short('b')
            .long("bind")
            .conflicts_with("connect")
            .help("binds connie to local_ip:4443 tcp"))
        .arg(Arg::new("connect")
            .long("connect"))
        .arg(Arg::new("verbose").long("verbose"))    
        .get_matches();
    Ok(match_result)
} 
