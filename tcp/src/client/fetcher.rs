use std::{io::Result, net::{IpAddr, SocketAddr}};

use lib_db::{
    media::fetch::Smedia,
    sqlite::{
        sqlite_host::SqliteHost,
        sqlite_user::SqliteUser
    }
};
use common_lib::{
    log::debug,
    public_ip,
    tokio::{
        io::{
            AsyncReadExt,
            AsyncWriteExt
        },
        net::TcpStream
    }
};
use tokio_rustls::rustls::pki_types::ServerName;
use crate::{
    client::connector::get_tlstream, common::{request::FETCH, util::client::rvfs}, server::req_format::Chead
};


pub async fn get_files(
    u: SqliteUser,
    server: SqliteHost,
    jwt: String,
) -> Result<()> {
  let port = server.port;
    let me_pub_ip = public_ip::addr().await;
    let ip: IpAddr;
        let addr = if me_pub_ip.is_some() && me_pub_ip.unwrap().to_string() != server.pub_ip {
            ip = server.pub_ip.parse().unwrap();
            SocketAddr::new(server.pub_ip.parse().unwrap(), port)
        } else {
            ip = server.pri_ip.parse().unwrap();
            SocketAddr::new(server.pri_ip.parse().unwrap(), port) 
        };
    let stream = TcpStream::connect(&addr).await?;
    let server_name = ServerName::from(ip);
    let mut stream = get_tlstream(server_name, stream).await?;
    let head = Chead {
        jwt,
        cpid: u.cpid.clone()
    };
    let request = head.sz().unwrap(); 
    stream.write_u8(FETCH).await?;
    stream.write_all(&request).await?;
    debug!("sent {}",request.len());
    stream.flush().await?;
    let items = stream.read_u16().await.unwrap();

    println!("media");
    for _i in 1..items {
        let buf = rvfs(&mut stream).await?;
        let media: Smedia = Smedia::dz(buf).unwrap();
        println!("{_i}: name: {},\n size: {}, checksum: {} ;",media.name, media.size, media.checksum);
        // let sqlitem = SqliteMedia {
        //     name: media.name,
        //     cpid: u.cpid.clone(),
        //     host: server.cpid.clone(),
        //     path
        // }
    }


    Ok(())
}
