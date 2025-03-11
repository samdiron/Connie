

#[allow(dead_code)]
pub(crate) mod request;


#[allow(dead_code)]
pub(crate) mod handshakes {
    use std::io;

    use common_lib::log::{debug, warn};
    use lib_db::{sqlite::sqlite_host::SqliteHost, types::SqlitePool};
    use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

    use crate::common::util::{rvfs, wvts};

    /// makes sure the client is in the correct addres and takes the public ip of the server if
    /// it's correct and returns 0 if the server is correct if not 1
    pub async fn client(
        stream: &mut TcpStream,
        server: &SqliteHost,
        pool: &SqlitePool
    ) -> Result<u8, io::Error>  {

        debug!("START:HANDSHAKE");
        debug!("HANDSHAKING:{}", server.name);
        let server_name = server.name.as_bytes();
        let _res = wvts(stream, server_name.to_vec()).await?;
        debug!("sent server_name with {_res}");
        let server_confirm = stream.read_u8().await?;
        if server_confirm != 0 {debug!("HANDSHAKE:FAILD:SERVER did not confirm");return Ok(1)};
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
            }else {warn!("the server on this addres is not {}", server.name);}
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
        stream: &mut TcpStream,
        server: &SqliteHost
    ) -> Result<u8, io::Error> {
        debug!("START:HANDSHAKE");
        let buf = rvfs(stream).await?;
        let lossy = buf.to_vec();
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
        if client_confirm == 0 {
            wvts(
                stream,
                server.pub_ip
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
    use std::time;

    use common_lib::{log::{debug, info}, tokio::{
        fs::File,
        io::{
            AsyncReadExt,
            AsyncWriteExt,
            BufReader,
            BufWriter,
            Result
        },
    }};
    use tokio::net::TcpStream;
    use super::request::PACKET_SIZE;
    // reads the amount of b from a stream and returns the data read in a Vec<u8>
    // this function is made only for small reads it will not work as expected with larg buffers
    pub async fn read_stream(
        s: &mut TcpStream,
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
        s: &mut TcpStream,
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
        s: &mut TcpStream,
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
        s: &mut TcpStream,
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
                if _when_to_print == 3 {_when_to_print=0};
                if _when_to_print == 0 {
                    let percent = (i as f64 / tol as f64) * 100.0;
                    info!("upload: {:.2}%", percent);
                };
                _when_to_print+=1;
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
        let end = time::Instant::now();
        assert_eq!(_usize , sent);
        let mb = (sent as f64 / 1000 as f64) / 1000 as f64;
        let dur = (end-start).as_secs_f64();
        info!(
            "RW: sent {:.2}MB in {:.2}s Average speed {:.2}Mb/s",
            mb, dur, mb/dur
        );


        info!("RW: waiting for confirm to end request");
        s.write_u8(0).await?;
        
        Ok(sent)
    }
    
    /// reads a standard (PACKET_SIZE) from stream and 
    /// writes the buffer into file
    pub async fn wifb(
        s: &mut TcpStream,
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
            let mut when_to_print:u8 = 0;
            loop {
                if i == tol || i == tol-1 || (recvd+PACKET_SIZE) > s_all {
                    debug!("last loop");break 
                }
                s.read_exact(&mut buf).await?;
                writer.write_all(&buf).await?;
                writer.flush().await?;
                i+=1;
                if when_to_print == 3 {
                    when_to_print = 0;
                }

                if when_to_print == 0 {
                    let percent = (i as f64 / tol as f64) * 100.0;
                    info!("download: {:.2}%", percent);
                };
                when_to_print+=1;
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
        let status = s.read_u8().await?;
        let end = time::Instant::now();
        let mb = (s_all as f64 / 1000 as f64) / 1000 as f64;
        let dur = (end-start).as_secs_f64();
        info!(
            "RW: received {:.2}MB in {:.2}s Average speed {:.2}Mb/s",
            mb, dur, mb/dur
        );
        Ok((status, wrote))
    }

}
