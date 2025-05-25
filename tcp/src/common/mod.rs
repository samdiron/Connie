#[allow(dead_code)]
pub(crate) mod request;
pub(crate) mod util;

use tokio_rustls::server;
use tokio_rustls::client;
use tokio::net::TcpStream;


pub(crate) type ServerTlsStreams = server::TlsStream<TcpStream>;
pub(crate) type ClientTlsStreams = client::TlsStream<TcpStream>; 




#[allow(dead_code)]
pub(crate) mod handshakes {
    use std::io;
    use std::time::Duration;
    
    use common_lib::log::{debug, warn};
    use common_lib::public_ip;
    use common_lib::tokio::io::{AsyncReadExt, AsyncWriteExt};
    
    use lib_db::{
        sqlite::sqlite_host::SqliteHost,
        types::SqlitePool
    };
    use tokio::net::TcpStream;
    use tokio::time::timeout;
    
    use crate::common::{
        request::SIGNIN_CRED,
        util
    };

    use crate::common::{ClientTlsStreams, ServerTlsStreams};

    /// makes sure the client is in the correct addres and takes the public ip of the server if
    /// it's correct and returns 0 if the server is correct if not 1
    pub async fn client(
        stream: &mut ClientTlsStreams,
        server: &SqliteHost,
        pool: &SqlitePool
    ) -> Result<u8, io::Error>  {
        use util::client::{wvts, rvfs};

        debug!("START:HANDSHAKE");
        debug!("HANDSHAKING:{}", server.name);
        let server_name = server.name.as_bytes();
        let _res = wvts(Some(stream), None, server_name.to_vec()).await?;
        debug!("sent server_name with {_res}");
        let server_confirm = stream.read_u8().await?;
        if server_confirm != 0 {
            debug!("HANDSHAKE:FAILD:SERVER did not confirm");
            return Ok(1)
        };
        debug!("MID:HANDSHAKE");
        let buf_cpid = rvfs(Some(stream), None).await?;
        let buf_cpid = buf_cpid.to_vec();
        let cpid = String::from_utf8_lossy(&buf_cpid);
        stream.write_u8(0).await?;
        stream.flush().await?;
        let buf_host = rvfs(Some(stream), None).await?;
        let buf_host = buf_host.to_vec();
        let host = String::from_utf8_lossy(&buf_host);
        if cpid != server.cpid {
            warn!("server confirmed name and sent a wrong cpid");
            stream.write_u8(1).await?;
            stream.flush().await?;
            if host == server.host {
                warn!("the server on this addres is not {}, but it's ont he same machine: {}", server.name, host);
            }else {
                warn!("the server on this addres is not {}", server.name);
            }
            return Ok(1)
        } else {
            debug!("END:HANDSHAKE");
            stream.write_u8(0).await?;
            stream.flush().await?;
            let buf_ip = rvfs(Some(stream), None).await?;
            let buf_ip = buf_ip.to_vec();
            let public_ip = String::from_utf8_lossy(&buf_ip);
            if public_ip != server.pub_ip {
                SqliteHost::update_pub_ip(
                    server,
                    public_ip.parse().unwrap(),
                    pool
                ).await.unwrap();
                
            };
            debug!("SUCCSESFUL:HANDSHAKE");
            return Ok(0)
        }


    }

    pub async fn raw_client_handshake(
        stream: &mut TcpStream,
        server: &SqliteHost,
        pool: &SqlitePool
    ) -> Result<u8, io::Error>  {
        use util::client::{wvts, rvfs};

        debug!("START:HANDSHAKE");
        debug!("HANDSHAKING:{}", server.name);
        let server_name = server.name.as_bytes();
        let _res = wvts(None, Some(stream),server_name.to_vec()).await?;
        debug!("sent server_name with {_res}");
        let server_confirm = stream.read_u8().await?;
        if server_confirm != 0 {
            debug!("HANDSHAKE:FAILD:SERVER did not confirm");
            return Ok(1)
        };
        debug!("MID:HANDSHAKE");
        let buf_cpid = rvfs(None, Some(stream)).await?;
        let buf_cpid = buf_cpid.to_vec();
        let cpid = String::from_utf8_lossy(&buf_cpid);
        stream.write_u8(0).await?;
        stream.flush().await?;
        let buf_host = rvfs(None, Some(stream)).await?;
        let buf_host = buf_host.to_vec();
        let host = String::from_utf8_lossy(&buf_host);
        if cpid != server.cpid {
            warn!("server confirmed name and sent a wrong cpid");
            stream.write_u8(1).await?;
            stream.flush().await?;
            if host == server.host {
                warn!("the server on this addres is not {}, but it's ont he same machine: {}", server.name, host);
            }else {
                warn!("the server on this addres is not {}", server.name);
            }
            return Ok(1)
        } else {
            debug!("END:HANDSHAKE");
            stream.write_u8(0).await?;
            stream.flush().await?;
            let buf_ip = rvfs(None, Some(stream)).await?;
            let buf_ip = buf_ip.to_vec();
            let public_ip = String::from_utf8_lossy(&buf_ip);
            if public_ip != server.pub_ip {
                SqliteHost::update_pub_ip(
                    server,
                    public_ip.parse().unwrap(),
                    pool
                ).await.unwrap();
                
            };
            debug!("SUCCSESFUL:HANDSHAKE");
            return Ok(0)
        }


    }
    /// this function assures the client that is 
    /// in the correct addres and send the pub ip of
    /// the server and if the if the client is in the correct addres it will return 0 else 1
    pub async fn server(
        stream: &mut ServerTlsStreams,
        server: &SqliteHost
    ) -> Result<u8, io::Error> {
        use util::server::{wvts, rvfs};
        debug!("START:HANDSHAKE");
        
        let buf = rvfs(Some(stream), None).await?;
        let lossy = buf.to_vec();
        if lossy.len() == 1usize && lossy[0] == SIGNIN_CRED {
            return Ok(SIGNIN_CRED);
        }; 
        let name = String::from_utf8_lossy(&lossy);
        if name != server.name {
            debug!("FAILD:HANDSHAKE: {name}");
            stream.write_u8(1).await?;
            stream.flush().await?;
            return Ok(1)
        };
        stream.write_u8(0).await?;
        stream.flush().await?;
        wvts(
            Some(stream),
            None,
            server.cpid
                .as_bytes()
                .to_vec()
        ).await?;
        debug!("SENT:CPID");
        let _confirm = stream.read_u8().await?;
        wvts(
            Some(stream),
            None,
            server.host
                .as_bytes()
                .to_vec()
        ).await?;
        debug!("SENT:HOST");
        let client_confirm = stream.read_u8().await?;

        let dur = Duration::from_secs_f64(0.30);
        let server_ip_bind = timeout(dur ,public_ip::addr()).await;
        let server_ip = if server_ip_bind.is_ok() {
            server_ip_bind.unwrap().unwrap().to_string()
        }else { server.pub_ip.clone() };

        if client_confirm == 0 {
            wvts(
                Some(stream),
                None,
                server_ip
                    .as_bytes()
                    .to_vec()
            ).await?;
            debug!(":SUCCSESFUL");
            return Ok(0)
        } else {return Ok(1)}
    }




    pub async fn raw_server_handshake(
        stream: &mut TcpStream,
        server: &SqliteHost
    ) -> Result<u8, io::Error> {
        use util::server::{wvts, rvfs};
        debug!("START:HANDSHAKE");
        
        let buf = rvfs(None, Some(stream)).await?;
        let lossy = buf.to_vec();
        if lossy.len() == 1usize && lossy[0] == SIGNIN_CRED {
            return Ok(SIGNIN_CRED);
        }; 
        let name = String::from_utf8_lossy(&lossy);
        if name != server.name {
            debug!("FAILD:HANDSHAKE: {name}");
            stream.write_u8(1).await?;
            stream.flush().await?;
            return Ok(1)
        };
        stream.write_u8(0).await?;
        wvts(
            None,
            Some(stream),
            server.cpid
                .as_bytes()
                .to_vec()
        ).await?;
        debug!("SENT:CPID");
        let _confirm = stream.read_u8().await?;
        wvts(
            None,
            Some(stream),
            server.host
                .as_bytes()
                .to_vec()
        ).await?;
        debug!("SENT:HOST");
        let client_confirm = stream.read_u8().await?;

        let dur = Duration::from_secs_f64(0.30);
        let server_ip_bind = timeout(dur ,public_ip::addr()).await;
        let server_ip = if server_ip_bind.is_ok() {
            server_ip_bind.unwrap().unwrap().to_string()
        }else { server.pub_ip.clone() };

        if client_confirm == 0 {
            wvts(
                None,
                Some(stream),
                server_ip
                    .as_bytes()
                    .to_vec()
            ).await?;
            debug!(":SUCCSESFUL");
            return Ok(0)
        } else {return Ok(1)}

        
        
    }

}


