
use std::io::Result;
use std::process::exit;
use std::net::{IpAddr, SocketAddr};

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
    pool: &SqlitePool
) -> Result<Vec<Smedia>> {
    let port = server.port;
    let me_pub_ip = public_ip::addr().await;
    let ip: IpAddr;
        let addr = if me_pub_ip.is_some() 
        && me_pub_ip.unwrap().to_string() != server.pub_ip {
            ip = server.pub_ip.parse().unwrap();
            SocketAddr::new(server.pub_ip.parse().unwrap(), port)
        } else {
            ip = server.pri_ip.parse().unwrap();
            SocketAddr::new(server.pri_ip.parse().unwrap(), port) 
        };
    let stream = TcpStream::connect(&addr).await?;
    let server_name = ServerName::from(ip);
    let mut stream = get_tlstream(server_name, stream).await?;
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


    let request = head.sz().unwrap(); 
    stream.write_u8(FETCH).await?;
    stream.write_all(&request).await?;
    debug!("sent {}",request.len());
    stream.flush().await?;
    let items = stream.read_u16().await.unwrap();
    let mut media_from_server: Vec<Smedia> = vec![];

    for _i in 0..items {
        
        let buf = rvfs(&mut stream).await?;
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
