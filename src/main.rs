use clap::{command,ArgGroup, Arg, ArgMatches, Command};
use lib_start::init::{start,check_pid_lockfile};
use tokio::io::join;
use tokio::join;
use tokio::{
    spawn,
    runtime::Runtime,
    
};
//args start connect bind verbose sql max-clients only-users web-host  
fn main() {
    let match_result: ArgMatches = command!()
        .about("Connie is a private home server used to connect with other connie servers privately it can stream movies music and store private files ")
        .group(ArgGroup::new("server-user"))
        .arg(Arg::new("start")
            .short('s')
            .long("start")
            .help("ready to connect/bind mode"))
        // .arg(Arg::new("bind")
        //     .short('b')
        //     .long("bind")
        //     .conflicts_with("connect")
        //     .help("binds connie to local_ip:4443 tcp"))
        // .arg(Arg::new("connect")
        //     .short("c")
        //     .long("connect"))
        // .arg(Arg::new("verbose").short("v").long("verbose"))    
        .get_matches();
    
    // make a PID lock filei in start
     let rt = Runtime::new().unwrap();

    let is_ready : bool =  match_result.get_on::<bool>("start").unwrap();
    if is_ready{
    let _machine = rt.block_on(start()).expect("could not get machine info");
    println!("connie is up and ready");
    let matches = rt.block_on(commands()).expect("could not get user input");
           
    
    }
}


async fn commands() -> Result<(ArgMatches)> {
let match_result: ArgMatches = command!()
        .about("Connie is a private home server used to connect with other connie servers privately it can stream movies music and store private files ")
        .group(ArgGroup::new("server-user"))
       .arg(Arg::new("bind")
            .short('b')
            .long("bind")
            .conflicts_with("connect")
            .help("binds connie to local_ip:4443 tcp"))
        .arg(Arg::new("connect")
            .short("c")
            .long("connect"))
        .arg(Arg::new("verbose").short("v").long("verbose"))    
        .get_matches();
    Ok(match_result);
} 
