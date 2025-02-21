
#[allow(dead_code)]
pub(crate) mod request;
pub(crate) mod handshake;


#[allow(dead_code)]
pub(crate) mod util {
    use tokio::{
        fs::File,
        io::{
            AsyncReadExt,
            AsyncWriteExt,
            BufReader,
            BufWriter,
            Result
        },
        net::TcpStream
    };
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
        println!("STREAMREAD: bytes read {:?}", rcve);
        Ok(buf)
    } 
    /// stands for read vector from stream 
    /// and it only works with wvts
    pub async fn rvfs (
        s: &mut TcpStream,
    ) -> Result<Vec<u8>> {
        let buf_size = s.read_u16().await?;
        let mut buf = vec![0; buf_size as usize];
        s.read_exact(&mut buf).await?;
        s.write_u8(0).await.unwrap();
        Ok(buf)
        
    }
    /// stands for write vectr into stream 
    /// and only works with rvfs
    /// make sure the input buffer is less than a standard paket size
    pub async fn wvts(
        s: &mut TcpStream,
        mut fbuf: Vec<u8>
    ) -> Result<u8> {
        let all = fbuf.len();
        let sized = (all as u16);
        s.write_u16(sized).await?;
        s.write_all(&fbuf).await?;
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
        reader: &mut BufReader<File>
    ) -> Result<usize> {
        let mut nbuf = vec![0; PACKET_SIZE];
        let mut sent = 0usize;
        let _usize = _size as usize;
        let _tol = _size as f64 / PACKET_SIZE as f64;
        let tol = _tol.ceil() as u16;
        s.write_u16(tol).await?;
        s.flush().await?;
        s.write_u64(_size).await?;
        s.flush().await?;
        
        println!("tol: {tol}, size: {_size}");
        for i in 0..tol {

            if i == tol || tol == 1 || ((_usize - sent) < PACKET_SIZE){
                println!("end tol");
                let buf_size = _usize - sent;
                println!("buf_size: {buf_size}");
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
        println!("RW: sent: {sent} in {tol} iter;");
        println!("RW: waiting for confirm to end request");
        s.write_u8(0).await?;
        
        Ok(sent)
    }
    
    /// reads a standard (PACKET_SIZE) from stream and 
    /// writes the buffer into file
    pub async fn wifb(
        s: &mut TcpStream,
        writer: &mut BufWriter<File>
    ) -> Result<(u8, usize)> {
        let mut wrote = 0usize;
        let mut recvd = 0usize;
        let mut buf = vec![0; PACKET_SIZE];
        let tol = s.read_u16().await?;
        let s_all = s.read_u64().await? as usize;
        let mut i = 0u16;
        if i < tol && tol != 1  {
            loop {
                if i == tol || i == tol-1 || (recvd+PACKET_SIZE) > s_all {println!("break");break }
                s.read_exact(&mut buf).await?;
                writer.write_all(&buf).await?;
                writer.flush().await?;
                i+=1;
            }
        };

        if tol == 1 || i <= tol || (recvd+PACKET_SIZE) > s_all {
            println!("end tol");
            let buf_size = s.read_u16().await? as usize;
            let mut buf = vec![0; buf_size];
            println!("buf_size: {buf_size}");
            s.read_exact(&mut buf).await?;
            println!("read size: {buf_size}");
            recvd+=buf_size;
            writer.write_all(&buf).await?;
            writer.flush().await?;
            wrote+=buf_size;
            println!("buf_size: {buf_size}");
        };
        assert_eq!(wrote, recvd);
        let status = s.read_u8().await?;

        Ok((status, wrote))
    }

}
