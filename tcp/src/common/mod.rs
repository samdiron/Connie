
#[allow(dead_code)]
pub(crate) mod request;
pub(crate) mod handshake;


#[allow(dead_code)]
pub(crate) mod util {
    use std::io::Result;
    use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter}, net::TcpStream};

    use super::request::PACKET_SIZE;
    // reads the amount of b from a stream and returns the data read in a Vec<u8>
    // this function is made only for small reads it will not work as expected with larg buffers
    pub async fn read_stream(
        s: &mut TcpStream,
        b: u16
    ) -> Result<Vec<u8>> {
        let mut buf = vec![0; b as usize];
        loop {
            let size = s.read(&mut buf).await?;
            if size == 0usize {
                break;
            }
        }
        Ok(buf)
    } 
    /// stands for write from file buffer 
    /// it reads from the file a standard size buffer (PACKET_SIZE)
    /// then i writes it to a stream
    pub async fn wffb(
        s: &mut TcpStream,
        size: i64,
        reader: &mut BufReader<File>
    ) -> Result<(u8, usize)> {
        let mut buf = vec![0; PACKET_SIZE];
        let mut sent = 0usize;

        loop {
            let _res = reader.read_buf(&mut buf).await;
            if _res.is_ok() {
                let read = _res?;
                if 0usize == read {
                    break;
                }
                let size = s.write(&buf).await?;
                if size == 0usize {
                    break;
                }
                sent+=size;
            }
        }
        s.flush().await?;
        assert_eq!(size as usize , sent);
        let status = s.read_u8().await?;
        
        Ok((status, sent))
    }
    
    /// reads a standard (PACKET_SIZE) from stream and 
    /// writes the buffer into file
    pub async fn read_into_file_buf(
        s: &mut TcpStream,
        writer: &mut BufWriter<File>
    ) -> Result<(u8, usize)> {
        let mut wrote = 0usize;
        let mut recvd = 0usize;
        let mut buf = vec![0; PACKET_SIZE];

        loop {
            let size = s.read(&mut buf).await?;
            if 0usize == 0usize {
                break;
            }
            recvd+=size;
            writer.write_all(&buf).await?;
            wrote+=size;
        }
        assert_eq!(wrote, recvd);
        let status = s.read_u8().await?;

        Ok((status, wrote))
    }

}
