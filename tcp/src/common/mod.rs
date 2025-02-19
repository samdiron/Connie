
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

    pub async fn wvts(
        s: &mut TcpStream,
        mut fbuf: Vec<u8>
    ) -> Result<usize> {
        let all = fbuf.len();
        let mut sent = 0usize;
        loop {
            fbuf.remove(sent);
            let size = s.write(&fbuf).await?;
            sent+=size;
            if sent == all {break;};
        }
        assert_eq!(sent, all);
        Ok(sent)
        
    }
    /// stands for write from file buffer 
    /// it reads from the file a standard size buffer (PACKET_SIZE)
    /// then i writes it to a stream
    pub async fn wffb(
        s: &mut TcpStream,
        _size: i64,
        reader: &mut BufReader<File>
    ) -> Result<usize> {
        let mut buf = vec![0; PACKET_SIZE];
        let mut sent = 0usize;
        let mut i = 0;
        loop {
            println!("i");
            let _res = reader.read(&mut buf).await;
            println!("i");
            if _res.is_ok() {
                let read = _res?;
                if 0usize == read {
                    break;
                }
                if read < PACKET_SIZE && (read == _size as usize ){buf.resize_with(read, Default::default);}
                let size = s.write(&buf).await?;
                println!("i");
                if size == 0usize {
                    break;
                }
                i+=1;
                sent+=size;
                println!("sending: {size}");
            }
        }
        s.flush().await?;
        assert_eq!(_size as usize , sent);
        println!("RW: sent: {sent} in {i} iter;");
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
        let mut i: u8 = 1; 
        loop {
            if i == 0 {break;}
            println!("i");
            let size = s.read(&mut buf).await?;
            if 0usize == size  {
                break;
            }
            
            println!("i");
            recvd+=size;
            if size < PACKET_SIZE {
                buf.resize_with(size, Default::default);
                println!("rszd");
                i=0
            }
            println!("i: new buf {}",buf.len());
            let wrt = writer.write(&buf).await?;
            writer.flush().await?;
            println!("i: wote {wrt}");
            wrote+=wrt;
            println!("ie");
        }
        assert_eq!(wrote, recvd);
        let status = s.read_u8().await?;

        Ok((status, wrote))
    }

}
