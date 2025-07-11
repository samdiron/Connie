const WAIT_FOR_UPDATE_PERCENTAGE: u16 = 1;

use std::time::{
    self,
    Instant,
    Duration,
};

use common_lib::{
    display_size,
    log::{debug, info}, 
    tokio::{
        net::TcpStream,
        fs::File,
        io::{
            AsyncReadExt,
            AsyncWriteExt,
            BufReader,
            BufWriter,
            Result
        },
}};
use crate::common::request::PACKET_SIZE;



pub async fn raw_read_stream(
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
    debug!("STREAMREAD: bytes read {:?}", rcve);
    Ok(buf)
}

/// stands for read vector from stream 
/// and it only works with wvts
pub async fn raw_rvfs (
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
pub async fn raw_wvts(
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

/// a raw tcp version of wffb(write into file buffer)
/// faster and takes less processing power
/// but not secure
/// returns (bits written' to stream: usize)
/// fails if data sent does not match data written 
pub async fn raw_wffb(
    s: &mut TcpStream,
    _size: u64,
    reader: &mut BufReader<File>,
    verbose: bool
) -> Result<usize> {
    let start = time::Instant::now();
    let mut nbuf = vec![0; PACKET_SIZE];
    let mut sent = 0usize;
    let _usize = _size as usize;
    let _tol = _usize as f64 / PACKET_SIZE as f64;
    let tol = _tol.ceil() as u64;
    s.write_u64(tol).await?;
    s.flush().await?;
    s.write_u64(_size).await?;
    s.flush().await?;

    debug!("tol: {tol}, size: {_size}");
    let standard_wait = Duration::from_secs(WAIT_FOR_UPDATE_PERCENTAGE.into());
    let mut _when_to_print = Instant::now();

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
                        sent+=PACKET_SIZE;
        };

        if verbose && _when_to_print.elapsed() >= standard_wait {
            _when_to_print+=standard_wait;
            let percent = (i as f64 / tol as f64) * 100.0;
            info!("upload: {:.2}%", percent);
        };
    }
    s.flush().await?;
    assert_eq!(_usize , sent);
    let string_size = display_size(sent as u64);
    let mb  = (sent as f64 / 1000 as f64) / 1000 as f64;
    let dur = (start.elapsed()).as_secs_f64();
    // sendin confirmation  
    s.write_u8(0).await?;
    //
    info!(
        "RW: sent {} in {:.2}s Average speed {:.2}Mb/s",
        string_size, dur, mb/dur
    );
    Ok(sent)
}

/// a raw tcp version of wifb(write into file buffer)
/// faster and takes less processing power
/// but not secure
/// returns (status: u8, bits written' to disk : usize)
/// fails if data sent does not match data written 
pub async fn raw_wifb(
    s: &mut TcpStream,
    writer: &mut BufWriter<File>,
    verbose: bool
) -> Result<(u8, usize)> {

    let start = time::Instant::now();
    let mut wrote = 0usize;
    let mut recvd = 0usize;
    let mut buf = vec![0; PACKET_SIZE];
    let tol = s.read_u64().await?;
    let s_all = s.read_u64().await? as usize;
    let mut i = 0u64;
    let standard_wait = Duration::from_secs(WAIT_FOR_UPDATE_PERCENTAGE.into());

    if i < tol && tol != 1 {
        let mut when_to_print = Instant::now();
        loop {
            if i == tol || i == tol-1 || (recvd+PACKET_SIZE) > s_all {
                tui::restore_terminal();
                debug!("last loop");break 
            }
            s.read_exact(&mut buf).await?;
            writer.write_all(&buf).await?;
            // writer.flush().await?;
            i+=1;
            
            if verbose && when_to_print.elapsed() >= standard_wait {
                when_to_print+=standard_wait;
                let percent = (i as f64 / tol as f64) * 100.0;
                // gauge_sender
                //     .send(percent)
                //     .expect("could not send percent to the gauge thread ");
                info!("download: {:.2}%", percent);
            };
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
    let string_size = display_size(s_all as u64);
    let mb = (s_all as f64 / 1000 as f64) / 1000 as f64;
    info!(
        "RW: received {} in {:.2}s Average speed {:.2}Mb/s",
        string_size, dur, mb/dur
    );
    Ok((status, wrote))
}
