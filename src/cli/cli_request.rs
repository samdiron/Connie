use std::{
    io::{
        stdin,
        stdout,
        Write
    },
    net::IpAddr,
    path::PathBuf,
    process::exit,
    str::FromStr
};

use common_lib::{
    log::{debug, error, info},
    path::SQLITEDB_PATH,
    public_ip
};
use lib_db::sqlite::{
    self,
    sqlite_host::fetch_server,
    sqlite_jwt::delete_expd_jwt,
    sqlite_media::{
        fetch_all_media_from_host,
        SqliteMedia
    },
    sqlite_user::fetch_sqlite_user_with_server_cpid
};
use tcp::{
    client::{
        client::client_process,
        fetcher
    },
    types::{GET, POST, RQM}};

use crate::Commands;


pub async fn handle_cli_request(command: Commands) {
    match command {

        Commands::REQUEST { 
            ip,
            port,
            host,
            server_name,
            db,
            get,
            post,
            create_checksum: checksum,
            fetch_files, 
            user
        } => {
            if post.is_some() && get.is_some() {println!("you can't enter a get and post command");exit(1)}
            let db_path = if db.is_some() {
                let path = db
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                debug!("using db path: {}",&path);
                path
            } else {SQLITEDB_PATH.to_string()};
            let _pool = sqlite::get_sqlite_conn(&db_path)
                .await
                .unwrap();
            let pool = &_pool;
            delete_expd_jwt(pool).await;
            
            let server = if host.is_some() && server_name.is_some() {
                let server_name = server_name.unwrap();
                let server = fetch_server(&server_name, host, pool).await;
                server
            } else if let Some(name) = server_name {
                let server = fetch_server(&name, None, pool).await;
                server
            }else {
                error!("you need to enter a user - server_name if
                you have more than 1 server with the same name it's
                better to enter the host too");
                exit(1)

            };
            let checksum = if checksum.is_some() {
                checksum.unwrap()
            }else {true};

            
            let usr = fetch_sqlite_user_with_server_cpid(&user, &server.cpid, pool)
                    .await
                    .expect("could not fetch user");

            if fetch_files.is_some(){
                debug!("fetching files");
                let jwt = sqlite::sqlite_jwt::get_jwt(
                    &server.cpid,
                    &usr.cpid,
                    pool
                ).await.unwrap();
                if jwt.is_none() {
                    error!("a jwt was not found");

                }
                let jwt = jwt.unwrap();
                let me_pub_ip = public_ip::addr().await;
                let ip = if ip.is_some() {ip.unwrap()} 
                    else if me_pub_ip.is_some()&&
                    me_pub_ip.unwrap().to_string() == server.pub_ip {
                    IpAddr::from_str(&server.pri_ip).unwrap()
                }else {IpAddr::from_str(&server.pub_ip).unwrap()};
                fetcher::get_files(
                    usr,
                    server,
                    jwt,
                ).await.unwrap();
            } else if post.is_some() { 
              
                debug!("creating a checksum: {checksum}");
                let request: RQM = RQM::create(
                    post.unwrap(),
                    POST.to_string(),
                    usr.cpid.clone(),
                    checksum
                ).await.unwrap();
                let usr = fetch_sqlite_user_with_server_cpid(&user, &server.cpid, pool).await.unwrap();
                let res = client_process(
                    _pool,
                    usr,
                    server,
                    None,
                    request
                ).await.unwrap();
                println!("done {}", res);
            } else if get.is_some() {
                let _media_vec = fetch_all_media_from_host(&server.cpid, &usr.cpid, pool).await;
                if _media_vec.is_err() {
                    let e = _media_vec.err().unwrap();
                    error!("database error: {}",e.to_string());
                    info!("you don't have any files in said host");
                    exit(0)
                    
                }else {
                    let mv = _media_vec.unwrap();
                    let mut i = 0;

                    for m in &mv {
                        i+=1;
                        println!("{i}(name: {}\n type: {}\nsize: {:.2}MB checksum: {}\n)",
                            m.name,
                            m.type_,
                            (m.size as f64 / 1000 as f64) / 1000 as f64,
                            m.checksum
                        );
                    };
                    print!("enter the index of media you want: ");
                    stdout().flush().unwrap();
                    let mut buf =  String::new();
                    let size = stdin().read_line(&mut buf).unwrap();
                    let index = buf.trim_ascii_end() ;
                    println!("you chose {index}");
                    let user_index:u32 = index.parse().unwrap();
                    let index = user_index - 1;
                    let m: SqliteMedia = mv[index as usize ].clone();
                    let getp = get.unwrap();
                    let path = if getp == PathBuf::from("./") {
                        format!("./{}", &m.name)
                    } else {
                        let path: String;
                        if getp.is_dir() {
                            let string_path = getp.to_str().unwrap();
                            if string_path.ends_with("/") {
                                path = format!("{string_path}{}",m.name);
                            }else {
                                path = format!("{string_path}/{}",m.name);
                            };
                        } else {
                            path = getp.to_str().unwrap().to_string();
                        };
                        path


                    };
                    let request = RQM {
                        cpid: m.cpid,
                        name: m.name,
                        size: m.size,
                        type_: m.type_,
                        header: GET.to_string(),
                        chcksum: m.checksum,
                        path: Some(path),
                    };
                    let res = client_process(
                        _pool,
                        usr,
                        server,
                        Some(checksum),
                        request
                    ).await.unwrap();
                    info!("STATUS: {res}");
                }
            }
            else {
                error!("you did not enter a command to execute")
            }
        }

        _=> {}

    }
}
