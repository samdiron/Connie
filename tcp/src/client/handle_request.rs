use std::{io::{Error, ErrorKind, Result}, path::PathBuf, str::FromStr, usize};

use common_lib::path::DATA_DIR;
use lib_db::types::PgPool;
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter}, net::TcpStream};

use crate::common::request::{DELETE, GET, PACKET_SIZE, POST};


/// this function take only the raw request and does not send it you have to send the full request
/// before usin this function and input the raw request GET/POST/ etc
/// the raw request has to constain the path of the file to be posted
pub async fn handle_client_request(
    stream: &mut TcpStream,
    raw_request: String
) -> Result<u8> {
    if raw_request.contains(GET) {
        let mut request = raw_request;
        let name = request.split_off(GET.char_indices().count());
        let mut path_buf = PathBuf::from_str(DATA_DIR).unwrap();
        path_buf.push(name);
        
        let f = File::create_new(path_buf).await.unwrap();

        let mut writer = BufWriter::new(f);
        let mut buf = vec![0; PACKET_SIZE.into()];

        loop {
            let _res = stream.read_exact(&mut buf).await;
             
            if _res.is_ok() {
                
                writer.write_all(&buf).await?
                
            } else {
                let err = _res.unwrap_err();
                
                if err.kind() == ErrorKind::UnexpectedEof {
                    writer.write_all(&buf).await?;
                    writer.flush().await?;
                    break
                } else {
                    return  Err(err);
                }
            }
        }
        return Ok(0)


    } else if raw_request.contains(POST) {
        let mut request = raw_request;
        let path = request.split_off(POST.char_indices().count());
        let file_name = request.split("/").last().unwrap();
        let f = File::open(path).await?;
        let mut reader = BufReader::new(f);
        let mut buf = vec![0; PACKET_SIZE as usize];
        stream.write_all(file_name.as_bytes()).await?;
        stream.flush().await?;
        let mut  confirm_buf = String::new();
        stream.read_to_string(&mut confirm_buf).await?;
        if confirm_buf.as_str() != "000" {
            println!("server did not confirm");
            return Ok(113)
        }

        loop {
            let _res = reader.read_exact(&mut buf).await;
            if _res.is_ok() {
                stream.write_all(&buf).await?;
                stream.flush().await?;
            } else {
                let err = _res.unwrap_err();
                if err.kind() == ErrorKind::UnexpectedEof {
                    stream.write_all(&buf).await?;
                    stream.flush().await?;
                }
                break;
            }
            
        }


       return Ok(0); 

    }
    Ok(1)
}



