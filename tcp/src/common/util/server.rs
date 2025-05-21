use std::time::Instant;

use common_lib::display_size;
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
use tokio::net::TcpStream;

use crate::common::util::core::*;
use crate::common::{request::PACKET_SIZE, ServerTlsStreams};

type UniTls = ServerTlsStreams;

// reads the amount of b from a stream and returns the data read in a Vec<u8>
// this function is made only for small reads it will not work as expected with larg buffer
pub async fn read_stream(
    s: Option<&mut UniTls>,
    raw: Option<&mut TcpStream>,
    b: u16
) -> Result<Vec<u8>> {
    if raw.is_some() {
        let s = raw.unwrap();
        let res = raw_read_stream(s, b).await?;
        return Ok(res);
    };
    assert!(s.is_some());

    let s = s.unwrap();

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
    debug!("STREAMREAD: bytes read {:?}", rcve);
    Ok(buf)
} 
/// stands for read vector from stream 
/// and it only works with wvts
pub async fn rvfs (
    s: Option<&mut UniTls>,
    raw: Option<&mut TcpStream>
) -> Result<Vec<u8>> {
    if raw.is_some() {
        let s = raw.unwrap();
        let res = raw_rvfs(s).await?;
        return Ok(res);
    };
    assert!(s.is_some());

    let s = s.unwrap();

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
    s: Option<&mut UniTls>,
    raw: Option<&mut TcpStream>,
    fbuf: Vec<u8>
) -> Result<u8> {
    if raw.is_some() {
        let s = raw.unwrap();
        let res = raw_wvts(s, fbuf).await?;
        return Ok(res)
    };
    assert!(s.is_some());
    let s = s.unwrap();
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
    s: Option<&mut UniTls>,
    raw: Option<&mut TcpStream>,
    _size: u64,
    reader: &mut BufReader<File>,
) -> Result<usize> {
    if raw.is_some() {
        let s = raw.unwrap();
        let res = raw_wffb(s, _size, reader, false).await?;
        return Ok(res);
    };
    assert!(s.is_some());

    let s = s.unwrap();

    let start = Instant::now();
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
    for i in 0..tol {

        if i == tol || tol == 1 || ((_usize - sent) < PACKET_SIZE){
            let buf_size = _usize - sent;
            debug!("end tol: buf_size: {buf_size}");
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
    s.flush().await?;
    assert_eq!(_usize , sent);
    let mb = (sent as f64 / 1000 as f64) / 1000 as f64;
    let string_size = display_size(sent as u64);
    let dur = (start.elapsed()).as_secs_f64();

    //sending confirmation
    s.write_u8(0).await?;
    // 
    info!(
        "RW: sent {} in {:.2}s Average speed {:.2}Mb/s",
        string_size, dur, mb/dur
    );


    info!("RW: waiting for confirm to end request");
    Ok(sent)
}

/// reads a standard (PACKET_SIZE) from stream and 
/// writes the buffer into file
pub async fn wifb(
    s: Option<&mut UniTls>,
    raw: Option<&mut TcpStream>,
    writer: &mut BufWriter<File>,
) -> Result<(u8, usize)> {
    if raw.is_some() {
        let s = raw.unwrap();
        let res = raw_wifb(s, writer, false).await?;
        return Ok(res)
    };

    let s = s.unwrap();
    let start = Instant::now();
    let mut wrote = 0usize;
    let mut recvd = 0usize;
    let mut buf = vec![0; PACKET_SIZE];
    let tol = s.read_u16().await?;
    let s_all = s.read_u64().await? as usize;
    let mut i = 0u16;
    
    if i < tol && tol != 1 {
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
    let string_size = display_size(s_all as u64);
    info!(
        "RW: received {} in {:.2}s Average speed {:.2}Mb/s",
        string_size, dur, mb/dur
    );
    Ok((status, wrote))
}

