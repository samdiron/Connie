use std::{
    path::PathBuf,
    str::FromStr,
    io::Result,
    
};

use common_lib::path::PUBLIC_DATA_DIR;




fn check_if_dir_exst() -> Result<bool> {
    let path = PathBuf::from_str(PUBLIC_DATA_DIR).unwrap();
    Ok(path.exists())
}




