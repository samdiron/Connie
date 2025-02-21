use std::{io::Result, os::unix::fs::MetadataExt, path::Path};

use sha256::try_async_digest;
use tokio::fs::File;

pub async fn get_fsum(path: &str) -> Result<String> {
    let input = Path::new(path);
    let sum = try_async_digest(input).await?;
    return Ok(sum);
    
}

pub async fn get_size(path: &str) -> Result<i64> {
    let f = File::open(path).await?;
    let meta = f.metadata().await?;
    let size = meta.size() as i64;

    Ok(size)
}
