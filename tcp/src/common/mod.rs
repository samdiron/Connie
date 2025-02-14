pub(crate) mod request;
pub(crate) mod handshake;



pub(crate) mod util {
    use std::{io::Result};
    use tokio::{io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter}, net::TcpStream};

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
    




}
