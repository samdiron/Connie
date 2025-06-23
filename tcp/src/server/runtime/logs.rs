use std::{net::IpAddr, path::PathBuf, str::FromStr, time};

use common_lib::{log::{debug, error}, path::{CLIENT_LOG_F, UNAUTHORIZED_CLIINET_LOG_F}};
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncSeekExt, AsyncWriteExt}
};


async fn check_if_file_exist(path: &str) -> tokio::io::Result<File> {
    let path_buf = PathBuf::from_str(path).unwrap();
    
    if !path_buf.exists() {
        let f = File::create_new(&path_buf).await?;
        return Ok(f);
    }else {
        let f = OpenOptions::new()
            .write(true)
            .open(path_buf).await?;
        return Ok(f);
    }

    
}


pub async fn client_log(
    ip: IpAddr,
    uuid: &String,
    request_type: &str,
    status: u8,
) -> tokio::io::Result<u8> {
    debug!("client log process");
    let string_ip = ip.to_string();
    let time = time::Instant::now();
    let log_template = format!(
    "{:?}:( uuid: {uuid} from: {string_ip} request: {request_type} status: {status} )",
    time
);
    let mut f = check_if_file_exist(CLIENT_LOG_F).await?;
    let _size = f.seek(std::io::SeekFrom::End(0)).await?;
    let n_template = match _size {
        0 => log_template,
        _ => format!("\n{log_template}")

    };
    match f.write_all(n_template.as_bytes()).await{
        Ok(..) => {},
        Err(e) => {
            error!("client log process unsuccesful err: {:?} ", e.to_string());
        }
    }
    f.sync_all().await?;
    drop(f);
    Ok(0)
}


pub async fn unauthorized_client(
    ip: IpAddr,
) -> tokio::io::Result<u8> {
    let string_ip = ip.to_string();
    let time = time::Instant::now();

    let log_template = format!("{:?}:( ip: {string_ip})", time);
    let mut f = check_if_file_exist(UNAUTHORIZED_CLIINET_LOG_F).await?;
    f.write_all(log_template.as_bytes()).await?;
    Ok(0)
}
