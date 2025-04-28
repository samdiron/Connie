#[allow(dead_code)]
pub(crate) mod request;


use tokio_rustls::server;
use tokio_rustls::client;
use tokio::net::TcpStream;


pub(crate) type ServerTlsStreams = server::TlsStream<TcpStream>;
pub(crate) type ClientTlsStreams = client::TlsStream<TcpStream>; 




#[allow(dead_code)]
pub(crate) mod handshakes {
    use std::io;

    use common_lib::log::{debug, warn};
    use common_lib::public_ip;
    use common_lib::tokio::io::{AsyncReadExt, AsyncWriteExt};
    
    use lib_db::{
        sqlite::sqlite_host::SqliteHost,
        types::SqlitePool
    };
    
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
        let _res = wvts(stream, server_name.to_vec()).await?;
        debug!("sent server_name with {_res}");
        let server_confirm = stream.read_u8().await?;
        if server_confirm != 0 {
            debug!("HANDSHAKE:FAILD:SERVER did not confirm");
            return Ok(1)
        };
        debug!("MID:HANDSHAKE");
        let buf_cpid = rvfs(stream).await?;
        let buf_cpid = buf_cpid.to_vec();
        let cpid = String::from_utf8_lossy(&buf_cpid);
        stream.write_u8(0).await?;
        stream.flush().await?;
        let buf_host = rvfs(stream).await?;
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
            let buf_ip = rvfs(stream).await?;
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
        
        let buf = rvfs(stream).await?;
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
            stream,
            server.cpid
                .as_bytes()
                .to_vec()
        ).await?;
        debug!("HANDSHAKE:SENT:CPID");
        let _confirm = stream.read_u8().await?;
        wvts(
            stream,
            server.host
                .as_bytes()
                .to_vec()
        ).await?;
        debug!("HANDSHAKE:SENT:HOST");
        let client_confirm = stream.read_u8().await?;
        let server_ip = public_ip::addr().await.unwrap().to_string();
        if client_confirm == 0 {
            wvts(
                stream,
                server_ip
                    .as_bytes()
                    .to_vec()
            ).await?;
            debug!("SUCCSESFUL:HANDSHAKE");
            return Ok(0)
        } else {return Ok(1)}

        
        
    }

}



#[allow(dead_code)]
pub(crate) mod util {
    pub mod client {
            
        use std::time::{
            self,
            Instant,
            Duration,
        };

        use common_lib::{
            log::{debug, info},
            tokio::{
            fs::File,
            io::{
                AsyncReadExt,
                AsyncWriteExt,
                BufReader,
                BufWriter,
                Result
            },
        }};
        use crate::common::ClientTlsStreams;
        use crate::common::request::PACKET_SIZE;
        // reads the amount of b from a stream and returns the data read in a Vec<u8>
        // this function is made only for small reads it will not work as expected with larg buffers
        
        type UniTls = ClientTlsStreams;
        const WAIT_FOR_UPDATE_PERCENTAGE: u16 = 1;
        pub async fn read_stream(
            s: &mut UniTls,
            b: u16
        ) -> Result<Vec<u8>> {
            let mut buf = vec![0; b as usize];
            let mut rcve = 0usize;
            for _i in 0..1 {
                let size = s.read(&mut buf).await?;
                if size == 0usize {
                    break;
                }
                rcve+=size;
            }
            if buf.len() > rcve {
                buf.resize_with(rcve, Default::default);
            }
            info!("STREAMREAD: bytes read {:?}", rcve);
            Ok(buf)
        } 
        /// stands for read vector from stream 
        /// and it only works with wvts
        pub async fn rvfs (
            s: &mut UniTls,
        ) -> Result<Vec<u8>> {
            let buf_size = s.read_u16().await?;
            debug!("should read {buf_size}");
            let mut buf = vec![0; buf_size as usize];
            s.read(&mut buf).await?;
            s.write_u8(0).await.unwrap();
            Ok(buf)
            
        }
        /// stands for write vectr into stream 
        /// and only works with rvfs
        /// make sure the input buffer is less than a standard paket size
        pub async fn wvts(
            s: &mut UniTls,
            fbuf: Vec<u8>
        ) -> Result<u8> {
            let all = fbuf.len();
            assert!(all < PACKET_SIZE);

            let sized = all as u16;
            debug!("should write {all}");
            s.write_u16(sized).await?;
            s.flush().await?;
            s.write_all(&fbuf).await?;
            s.flush().await?;
            let state = s.read_u8().await?;
            assert_eq!(state, 0);
            Ok(state)
            
        }
        /// stands for write from file buffer 
        /// it reads from the file a standard size buffer (PACKET_SIZE)
        /// then i writes it to a stream
        pub async fn wffb(
            s: &mut UniTls,
            _size: u64,
            reader: &mut BufReader<File>,
            verbose: bool
        ) -> Result<usize> {
            let start = time::Instant::now();
            let mut nbuf = vec![0; PACKET_SIZE];
            let mut sent = 0usize;
            let _usize = _size as usize;
            let _tol = _size as f64 / PACKET_SIZE as f64;
            let tol = _tol.ceil() as u16;
            s.write_u16(tol).await?;
            s.flush().await?;
            s.write_u64(_size).await?;
            s.flush().await?;

            info!("tol: {tol}, size: {_size}");
            let standard_wait = Duration::from_secs(WAIT_FOR_UPDATE_PERCENTAGE.into());
            if verbose {

                let mut _when_to_print = Instant::now();
                for i in 0..tol {
                    if i == tol || tol == 1 || ((_usize - sent) < PACKET_SIZE){
                        info!("end tol");
                        let buf_size = _usize - sent;
                        info!("buf_size: {buf_size}");
                        let end_buffer_size = buf_size as u16; 
                        s.write_u16(end_buffer_size).await?;

                        let mut buf = vec![0;buf_size];
                        reader.read_exact(&mut buf).await?;
                        s.write_all(&buf).await?;
                        s.flush().await?;
                        sent+=buf_size;
                        break;
                        
                    }else {
                        reader.read_exact(&mut nbuf).await?;
                        s.write_all(&nbuf).await?;
                        sent+=PACKET_SIZE
                    };
                    if _when_to_print.elapsed() >= standard_wait {
                        _when_to_print+=standard_wait;
                        let percent = (i as f64 / tol as f64) * 100.0;
                        info!("upload: {:.2}%", percent);
                    };
                }
            }
            else {
                for i in 0..tol {

                    if i == tol || tol == 1 || ((_usize - sent) < PACKET_SIZE){
                        info!("end tol");
                        let buf_size = _usize - sent;
                        info!("buf_size: {buf_size}");
                        let end_buffer_size = buf_size as u16; 
                        s.write_u16(end_buffer_size).await?;

                        let mut buf = vec![0;buf_size];
                        reader.read_exact(&mut buf).await?;
                        s.write_all(&buf).await?;
                        s.flush().await?;
                        sent+=buf_size;
                        break;
                        
                    }else {
                        reader.read_exact(&mut nbuf).await?;
                        s.write_all(&nbuf).await?;
                        sent+=PACKET_SIZE
                    }
                }
            }
            s.flush().await?;
            assert_eq!(_usize , sent);
            let mb = (sent as f64 / 1000 as f64) / 1000 as f64;
            let dur = (start.elapsed()).as_secs_f64();
            // sendin confirmation  
            s.write_u8(0).await?;
            //
            info!(
                "RW: sent {:.2}MB in {:.2}s Average speed {:.2}Mb/s",
                mb, dur, mb/dur
            );
            Ok(sent)
        }
        
        /// reads a standard (PACKET_SIZE) from stream and 
        /// writes the buffer into file
        pub async fn wifb(
            s: &mut UniTls,
            writer: &mut BufWriter<File>,
            verbose: bool
        ) -> Result<(u8, usize)> {
            let start = time::Instant::now();
            let mut wrote = 0usize;
            let mut recvd = 0usize;
            let mut buf = vec![0; PACKET_SIZE];
            let tol = s.read_u16().await?;
            let s_all = s.read_u64().await? as usize;
            let mut i = 0u16;
            let standard_wait = Duration::from_secs(WAIT_FOR_UPDATE_PERCENTAGE.into());

            if i < tol && tol != 1 && verbose {
                let mut when_to_print = Instant::now();
                loop {
                    if i == tol || i == tol-1 || (recvd+PACKET_SIZE) > s_all {
                        tui::restore_terminal();
                        debug!("last loop");break 
                    }
                    s.read_exact(&mut buf).await?;
                    writer.write_all(&buf).await?;
                    writer.flush().await?;
                    i+=1;
                  
                    if when_to_print.elapsed() >= standard_wait {
                        when_to_print+=standard_wait;
                        let percent = (i as f64 / tol as f64) * 100.0;
                        // gauge_sender
                        //     .send(percent)
                        //     .expect("could not send percent to the gauge thread ");
                        info!("download: {:.2}%", percent);
                    };
                }
            };
            if i < tol && tol != 1 && !verbose {
                loop {
                    if i == tol || i == tol-1 || (recvd+PACKET_SIZE) > s_all {
                        debug!("last loop");break 
                    }
                    s.read_exact(&mut buf).await?;
                    writer.write_all(&buf).await?;
                    writer.flush().await?;
                    i+=1;
                }
            };

            if tol == 1 || i <= tol || (recvd+PACKET_SIZE) > s_all {
                let buf_size = s.read_u16().await? as usize;
                let mut buf = vec![0; buf_size];
                s.read_exact(&mut buf).await?;
                recvd+=buf_size;
                writer.write_all(&buf).await?;
                writer.flush().await?;
                wrote+=buf_size;
            };
            assert_eq!(wrote, recvd);
            let dur = (start.elapsed()).as_secs_f64();
            // server confirmation
            let status = s.read_u8().await?;
            // 
            let mb = (s_all as f64 / 1000 as f64) / 1000 as f64;
            info!(
                "RW: received {:.2}MB in {:.2}s Average speed {:.2}Mb/s",
                mb, dur, mb/dur
            );
            Ok((status, wrote))
        }

    }

    pub mod server {

        use std::time::{self, Duration, Instant};

        use common_lib::log::{debug, info};
        use common_lib::tokio::{
            fs::File,
            io::{
                AsyncReadExt,
                AsyncWriteExt,
                BufReader,
                BufWriter,
                Result
            },
        };

        use crate::common::{request::PACKET_SIZE, ServerTlsStreams};
        
        type UniTls = ServerTlsStreams;
        
        // reads the amount of b from a stream and returns the data read in a Vec<u8>
        // this function is made only for small reads it will not work as expected with larg buffer
        pub async fn read_stream(
            s: &mut UniTls,
            b: u16
        ) -> Result<Vec<u8>> {
            let mut buf = vec![0; b as usize];
            let mut rcve = 0usize;
            for _i in 0..1 {
                let size = s.read(&mut buf).await?;
                if size == 0usize {
                    break;
                }
                rcve+=size;
            }
            if buf.len() > rcve {
                buf.resize_with(rcve, Default::default);
            }
            info!("STREAMREAD: bytes read {:?}", rcve);
            Ok(buf)
        } 
        /// stands for read vector from stream 
        /// and it only works with wvts
        pub async fn rvfs (
            s: &mut UniTls,
        ) -> Result<Vec<u8>> {
            debug!("STREAMREAD: start");
            let buf_size = s.read_u16().await?;
            let mut buf = vec![0; buf_size as usize];
            s.read(&mut buf).await?;
            s.write_u8(0).await.unwrap();
            Ok(buf)
            
        }
        /// stands for write vectr into stream 
        /// and only works with rvfs
        /// make sure the input buffer is less than a standard paket size
        pub async fn wvts(
            s: &mut UniTls,
            fbuf: Vec<u8>
        ) -> Result<u8> {
            let all = fbuf.len();
            assert!(all < PACKET_SIZE);
            
            debug!("STREAMWRITE: start");
            let sized = all as u16;
            s.write_u16(sized).await?;
            s.flush().await?;
            s.write_all(&fbuf).await?;
            s.flush().await?;
            let state = s.read_u8().await?;
            assert_eq!(state, 0);
            debug!("STREAMWRITE: {all}");
            Ok(state)
            
        }
        /// stands for write from file buffer 
        /// it reads from the file a standard size buffer (PACKET_SIZE)
        /// then i writes it to a stream
        pub async fn wffb(
            s: &mut UniTls,
            _size: u64,
            reader: &mut BufReader<File>,
            verbose: bool
        ) -> Result<usize> {
            let start = time::Instant::now();
            let mut nbuf = vec![0; PACKET_SIZE];
            let mut sent = 0usize;
            let _usize = _size as usize;
            let _tol = _size as f64 / PACKET_SIZE as f64;
            let tol = _tol.ceil() as u16;
            s.write_u16(tol).await?;
            s.flush().await?;
            s.write_u64(_size).await?;
            s.flush().await?;
            
            info!("tol: {tol}, size: {_size}");
            if verbose {
                let standard_wait = Duration::from_secs(5);
                let mut when_to_print = Instant::now();
                for i in 0..tol {
                    let mut _when_to_print: u8 = 0;
                    if i == tol || tol == 1 || ((_usize - sent) < PACKET_SIZE){
                        info!("end tol");
                        let buf_size = _usize - sent;
                        info!("buf_size: {buf_size}");
                        let end_buffer_size = buf_size as u16; 
                        s.write_u16(end_buffer_size).await?;

                        let mut buf = vec![0;buf_size];
                        reader.read_exact(&mut buf).await?;
                        s.write_all(&buf).await?;
                        s.flush().await?;
                        sent+=buf_size;
                        break;
                        
                    }else {
                        reader.read_exact(&mut nbuf).await?;
                        s.write_all(&nbuf).await?;
                        sent+=PACKET_SIZE
                    };
                    if when_to_print.elapsed() >= standard_wait {
                        when_to_print+=standard_wait;
                        let percent = (i as f64 / tol as f64) * 100.0;
                        info!("upload: {:.2}%", percent);
                    };
                }
            }
            else {
                for i in 0..tol {

                    if i == tol || tol == 1 || ((_usize - sent) < PACKET_SIZE){
                        info!("end tol");
                        let buf_size = _usize - sent;
                        info!("buf_size: {buf_size}");
                        let end_buffer_size = buf_size as u16; 
                        s.write_u16(end_buffer_size).await?;

                        let mut buf = vec![0;buf_size];
                        reader.read_exact(&mut buf).await?;
                        s.write_all(&buf).await?;
                        s.flush().await?;
                        sent+=buf_size;
                        break;
                        
                    }else {
                        reader.read_exact(&mut nbuf).await?;
                        s.write_all(&nbuf).await?;
                        sent+=PACKET_SIZE
                    }
                }
            }
            s.flush().await?;
            assert_eq!(_usize , sent);
            let mb = (sent as f64 / 1000 as f64) / 1000 as f64;
            let dur = (start.elapsed()).as_secs_f64();

            //sending confirmation
            s.write_u8(0).await?;
            // 
            info!(
                "RW: sent {:.2}MB in {:.2}s Average speed {:.2}Mb/s",
                mb, dur, mb/dur
            );


            info!("RW: waiting for confirm to end request");
            Ok(sent)
        }
        
        /// reads a standard (PACKET_SIZE) from stream and 
        /// writes the buffer into file
        pub async fn wifb(
            s: &mut UniTls,
            writer: &mut BufWriter<File>,
            verbose: bool
        ) -> Result<(u8, usize)> {
            let start = time::Instant::now();
            let mut wrote = 0usize;
            let mut recvd = 0usize;
            let mut buf = vec![0; PACKET_SIZE];
            let tol = s.read_u16().await?;
            let s_all = s.read_u64().await? as usize;
            let mut i = 0u16;
            
            if i < tol && tol != 1 && verbose {
                let standard_wait = Duration::from_secs(5);
                let mut when_to_print = Instant::now();

                loop {
                    if i == tol || i == tol-1 || (recvd+PACKET_SIZE) > s_all {
                        debug!("last loop");break 
                    }
                    s.read_exact(&mut buf).await?;
                    writer.write_all(&buf).await?;
                    writer.flush().await?;
                    i+=1;

                    if when_to_print.elapsed() >= standard_wait {
                        when_to_print+=standard_wait;
                        let percent = (i as f64 / tol as f64) * 100.0;
                        info!("download: {:.2}%", percent);
                    };
                }
            };
            if i < tol && tol != 1 && !verbose {
                loop {
                    if i == tol || i == tol-1 || (recvd+PACKET_SIZE) > s_all {
                        debug!("last loop");break 
                    }
                    s.read_exact(&mut buf).await?;
                    writer.write_all(&buf).await?;
                    writer.flush().await?;
                    i+=1;
                }
            };

            if tol == 1 || i <= tol || (recvd+PACKET_SIZE) > s_all {
                let buf_size = s.read_u16().await? as usize;
                let mut buf = vec![0; buf_size];
                s.read_exact(&mut buf).await?;
                recvd+=buf_size;
                writer.write_all(&buf).await?;
                writer.flush().await?;
                wrote+=buf_size;
            };
            assert_eq!(wrote, recvd);
            let dur = (start.elapsed()).as_secs_f64();
            // client confirmation 
            let status = s.read_u8().await?;
            // 
            let mb = (s_all as f64 / 1000 as f64) / 1000 as f64;
            info!(
                "RW: received {:.2}MB in {:.2}s Average speed {:.2}Mb/s",
                mb, dur, mb/dur
            );
            Ok((status, wrote))
        }

    }
}
