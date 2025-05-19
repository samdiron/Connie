
use std::io::Result;
use std::process::exit;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use lib_db::{
    media::fetch::Smedia, 
    types::SqlitePool
};
use lib_db::sqlite::{
        sqlite_host::SqliteHost,
        sqlite_user::SqliteUser
};

use common_lib::{
    log::debug,
    public_ip,
};
use common_lib::tokio::{
        io::{
            AsyncReadExt,
            AsyncWriteExt
        },
        net::TcpStream
};

use tokio_rustls::rustls::pki_types::ServerName;

use crate::common::{
    handshakes,
    request::FETCH,
    util::client::rvfs,
};
use crate::{
    server::req_format::Chead,
    client::connector::get_tlstream,
};


pub async fn get_files(
    u: &SqliteUser,
    server: &SqliteHost,
    jwt: String,
    custom_port: Option<u16>,
    custom_ip: Option<IpAddr>,
    pool: &SqlitePool
) -> Result<Vec<Smedia>> {
    let port = if custom_port.is_some() {
        debug!("using custom port");
        custom_port.unwrap()
    } else {
        debug!("using server default port: {}",server.port);
        server.port
    };

    let server_pub_ip: IpAddr = server.pub_ip.parse().unwrap();
    let server_pri_ip: IpAddr = server.pri_ip.parse().unwrap();

    let bind_me_pub_ip = public_ip::addr().await;
    let me_pub_ip = if bind_me_pub_ip.is_some() {
        bind_me_pub_ip.unwrap()
    } else {
        IpAddr::from_str("0.0.0.0").unwrap()
    };
    debug!("current public ip: {:?}", me_pub_ip);

    let ip: IpAddr;
    let addr = if custom_ip.is_some() {
        debug!("using custom ip");
        ip = custom_ip.unwrap();
        SocketAddr::new(ip, port)

    } else if me_pub_ip != server_pub_ip {
        debug!("using public ip: {}", &server.pub_ip);
        ip = server_pub_ip;
        SocketAddr::new(ip, port)
        
    } else {
        debug!("using private ip: {}", &server.pri_ip);
        ip = server_pri_ip;
        SocketAddr::new(ip, port)
        
    };

    let mut stream = TcpStream::connect(&addr).await?;
    let server_name = ServerName::from(ip);
    //
    //write connection status 
    stream.write_u8(0).await?;
    
    let mut stream = get_tlstream(stream, server_name).await?;
    debug!("tls connected");
    // get request ready before handshake
    let head = Chead {
        jwt,
        cpid: u.cpid.clone()
    };
    // handshake 
    let is_who_server = handshakes::client(
        &mut stream,
        &server,
        pool
    ).await?;
    if is_who_server != 0 {
        exit(1);
    };
    debug!("handshake done");

    // serializeing request 
    let request = head.sz().unwrap();
    // writing request header
    stream.write_u8(FETCH).await?;
    stream.write_all(&request).await?;

    debug!("sent {}",request.len());
    stream.flush().await?;
    let items = stream.read_u16().await.unwrap();
    let mut media_from_server: Vec<Smedia> = vec![];

    for _i in 0..items {
        
        let buf = rvfs(Some(&mut stream), None).await?;
        let media: Smedia = Smedia::dz(buf).unwrap();
        let sqlitem = Smedia {
            name: media.name,
            type_: media.type_,
            checksum: media.checksum,
            size: media.size,
        };

        media_from_server.push(sqlitem);
    }


    Ok(media_from_server)
}
