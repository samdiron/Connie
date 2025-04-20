
use std::io::{
    stdin,
    Write,
    stdout,
};
use std::{
    net::IpAddr,
    str::FromStr,
    path::PathBuf,
    process::exit,
};

use common_lib::{
    public_ip,
    path::SQLITEDB_PATH,
    log::{debug, error, info},
};

use lib_db::hash_passwords;
use lib_db::sqlite::sqlite_jwt::check_jwt_status;
use lib_db::{
    types::SqlitePool,
    media::fetch::Smedia,
    jwt::get_current_timestamp,
};

use lib_db::sqlite::{
    self,
    sqlite_host::fetch_server,
    sqlite_jwt::delete_expd_jwt,
    sqlite_user::fetch_sqlite_user_with_server_cpid,
};
use lib_db::sqlite::sqlite_media::{
    SqliteMedia,
    sqlite_delete_media,
    sqlite_media_exists,
    fetch_all_media_from_host,
    fetch_all_media_from_host_smedia,
};

use tcp::client::client::client_login_process;
use tcp::{
    client::{
        fetcher,
        client::client_process,
    },
    types::{GET, POST, RQM},
};

use crate::{get_pass, Commands};


async fn file_checker(
    db_files: Vec<Smedia>,
    server_files: Vec<Smedia>,
    host: String,
    cpid: String,
    pool: &SqlitePool
) {
    let mut deleted = 0;
    let mut added = 0;
    // deleteing local sqlite media that is deleted from the server
    for df in db_files {
        if !server_files.contains(&df) {
            sqlite_delete_media(
                &host,
                &cpid,
                &df.checksum,
                pool
            ).await;
            deleted+=1;
        };
    }
    // adding new fils from server
    for sf in server_files {
        if !sqlite_media_exists(&host, &cpid, &sf.checksum, pool).await {
            let media = SqliteMedia {
                name: sf.name,
                checksum: sf.checksum,
                cpid: cpid.clone(),
                host: host.clone(),
                size: sf.size,
                type_: sf.type_,
                date: get_current_timestamp() as i64,
                path: "./".to_string()
            };
            SqliteMedia::add_media(media, pool).await.unwrap();
            added+=1;
        };
    }
    info!("FILECHECKER: added {added} files from server");
    info!("FILECHECKER: deleted {deleted} files server");
}

#[allow(unused_assignments)]
pub async fn handle_cli_request(command: Commands) {
    match command {

        Commands::REQUEST { 
            db,
            Ip: ip,
            get,
            all,
            host,
            user,
            Port: port,
            post,
            login,
            // Delete,
            Domain,
            server_name,
            fetch_files, 
            create_checksum: checksum,
        } => {
            let mut command_vec: Vec<bool> = vec![];
            if get.is_some() {
                command_vec.push(true);
            };
            if post.is_some() {
                command_vec.push(true);
            };
            if fetch_files.is_some() {
                command_vec.push(true);
            };
            // if Delete.is_some() {
            //     command_vec.push(true);
            // };
            if login.is_some() {
                command_vec.push(true);
            }

            if command_vec.len() > 1usize {
                error!(
                    "you can't enter more than one request \n
                    (post/get/delete/fetch/login);"
                );
                exit(1);
            };
            if command_vec.is_empty() {
                error!("you need to enter a request");
                exit(1);
            };


            let db_path = if db.is_none() {
                debug!("using default sqliteDB ");
                SQLITEDB_PATH.to_string()
            } else {
                let bind = db.unwrap();
                let stred = bind.to_str().unwrap();
                let path = stred.to_string();
                debug!("using db path: {}",&path);
                path
            };
            let _pool = sqlite::get_sqlite_conn(&db_path.to_string())
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
            let checksum = checksum.unwrap();

            
            let usr = fetch_sqlite_user_with_server_cpid(
                &user,
                &server.cpid,
                pool
            ).await.expect("could not fetch user");

            if fetch_files.is_some(){
                let status = check_jwt_status(&usr.cpid, &server.cpid, pool).await;
                if !status {
                    error!("you have no available token please login first");
                    exit(1)
                };
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

                let ip = if ip.is_some() {
                    ip.unwrap()
                } else if ( me_pub_ip.is_some() ) &&
                    ( me_pub_ip.unwrap().to_string() == server.pub_ip ) {
                    IpAddr::from_str(&server.pri_ip).unwrap()
                }else {
                    IpAddr::from_str(&server.pub_ip).unwrap()
                };

                let host_cpid = server.cpid.clone();
                let user_cpid = usr.cpid.clone();
                let server_media = fetcher::get_files(
                    &usr,
                    &server,
                    jwt,
                    pool
                ).await.unwrap();

                if server_media.len() == 0usize {
                    info!("you don't have any files on that server");
                    exit(0)
                };
                let should_be_files = server_media.len() as u64;

                let db_media = fetch_all_media_from_host_smedia(
                    &host_cpid,
                    &user_cpid,
                    pool
                ).await.unwrap();

                file_checker(
                    db_media,
                    server_media,
                    host_cpid,
                    user_cpid,
                    pool
                ).await;
                info!("done");
                

            } else if post.is_some() { 
                let status = check_jwt_status(&usr.cpid, &server.cpid, pool).await;
                if !status {
                    error!("you have no available token please login first");
                    exit(1)
                };
                debug!("creating a checksum: {checksum}");
                let request: RQM = RQM::create(
                    post.unwrap(),
                    POST.to_string(),
                    usr.cpid.clone(),
                    checksum
                ).await.unwrap();

                let request = Some(request);
                let usr = fetch_sqlite_user_with_server_cpid(
                    &user,
                    &server.cpid,
                    pool
                ).await.unwrap();

                let res = client_process(
                    _pool,
                    usr,
                    server,
                    port,
                    ip,
                    None,
                    request,
                ).await.unwrap();

                println!("done {}", res);
            } else if get.is_some() {
                let status = check_jwt_status(&usr.cpid, &server.cpid, pool).await;
                if !status {
                    error!("you have no available token please login first");
                    exit(1)
                };
                let _media_vec = fetch_all_media_from_host(
                    &server.cpid,
                    &usr.cpid,
                    pool
                ).await;
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
                        println!("{i}(name: {}\n type: {}\nsize: {:.4}MB checksum: {}\n)",
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

                    let index = buf.trim_ascii_end();
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

                    let request = Some(request);
                    let res = client_process(
                        _pool.clone(),
                        usr.clone(),
                        server.clone(),
                        port,
                        ip,
                        Some(checksum),
                        request.clone(),

                    ).await.unwrap();

                    let res = if res == 8 {
                        info!("will connect again");
                        client_process(
                            _pool,
                            usr,
                            server,
                            port,
                            ip,
                            Some(checksum),
                            request,
                        ).await.unwrap()

                    }else {res};
                    info!("STATUS: {res}");
                }
            } else if login.is_some() && login.unwrap() {
                let mut passwd = String::new();
                let pass = get_pass(&mut passwd, &usr.name);
                let passwd = hash_passwords(passwd);
                client_login_process(
                    pool,
                    usr,
                    server,
                    port,
                    ip,
                    passwd
                ).await.unwrap();
            }
            else {
                error!("you did not enter a command to execute")
            }
        }

        _=> {}

    }
}
