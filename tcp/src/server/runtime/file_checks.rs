use std::{
    path::PathBuf,
    str::FromStr
};

use common_lib::{
    log::debug,
    path::DATA_DIR
};
use lib_start::file_checker;
use lib_db::{
    media::server_side::{
        in_storage_files,
        in_storage_size
    },
    types::PgPool
};


pub async fn clean_unfinished_files(
    cpid: &String,
    pool: &PgPool
) {    
    let files_size = in_storage_size(
        pool,
        cpid
    ).await;

    let files_path = in_storage_files(
        pool,
        cpid
    ).await;

    debug!("should be files: {}",files_path.len());
    let dir = PathBuf::from_str(DATA_DIR).unwrap();
    file_checker(&dir, &files_path, files_size).await;

}
