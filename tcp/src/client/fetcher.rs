use std::{io::Result, net::{IpAddr, SocketAddr}};

use lib_db::{media::fetch::Smedia, user::user_struct::User};
use serde::Deserialize;
use common_lib::tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use crate::{
    common::{request::FETCH, util::rvfs}, server::req_format::Chead
};


pub async fn get_files(u: User, ip: IpAddr, port: u16, jwt: String) -> Result<()> {
    let addr = SocketAddr::new(ip, port);
    let mut stream = TcpStream::connect(addr).await?;
    let head = Chead {
        jwt,
        cpid: u.cpid
    };
    let request = head.sz().unwrap(); 
    stream.write_u8(FETCH).await?;
    stream.write_all(&request).await?;
    stream.flush().await?;
    let items = stream.read_u16().await.unwrap();

    println!("media");
    for _i in 1..items {
        let buf = rvfs(&mut stream).await?;
        let media: Smedia = Smedia::dz(buf).unwrap();
        println!("{_i}: name: {}, size: {}, checksum: {} ;",media.name, media.size, media.checksum);

    }


    Ok(())
}
