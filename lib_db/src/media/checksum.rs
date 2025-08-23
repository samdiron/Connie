use std::{io::Result, path::Path};

use crate::sha256::try_async_digest;
use std::fs;

pub async fn get_fsum(path: &str) -> Result<String> {
    let input = Path::new(path);
    let sum = try_async_digest(input).await?;
    return Ok(sum);
    
}

pub async fn get_size(path: &str) -> Result<i64> {
    let meta = fs::metadata(path)?;
    let size = meta.len() as i64;

    Ok(size)
}
