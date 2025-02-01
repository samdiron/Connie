use std::{io::Result, os::unix::fs::MetadataExt};

use sha256::try_digest;
use tokio::fs::File;

pub async fn get(path: &str) -> Result<String> {
    let sum = try_digest(path)?;
    return Ok(sum);
    
}

pub async fn get_size(path: &str) -> Result<i64> {
    let f = File::open(path).await?;
    let meta = f.metadata().await?;
    let size = meta.size() as i64;

    Ok(size)
}
