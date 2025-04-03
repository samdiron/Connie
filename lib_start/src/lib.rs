#![allow(dead_code)]
use std::path::PathBuf;

use common_lib::{
    log::{debug, info},
    sysinfo, tokio::fs::remove_file,
};

pub mod init;
pub mod certs;
pub mod tcp;
pub(crate) mod check_path;
pub(crate) mod checks;


pub async fn file_checker(
    dir: &PathBuf,
    files: &Vec<PathBuf>,
    size: u64)  {
    let (current_files, ..) = get_files_in_storage(dir);
    debug!("FILE_CHECKER: found {} files in {:#?}",current_files.len(), dir);
    let mut files_removed = 0;
    if files.len() > 0usize {
        for i in 0usize..current_files.len() {
            if !files.contains(&current_files[i]) {
                remove_file(&current_files[i])
                    .await
                    .expect("could not remove file");
                debug!("FILE_CHECKER: removed file {:#?}", current_files[i]);
                files_removed+=1;
            }
        }
    }
    let (.., nsize) = get_files_in_storage(&dir);
    assert_eq!(size, nsize);
    info!("FILE_CHECKER: removed {files_removed} from {:#?}", dir);
}


/// return the available_space in all storage 
fn check_storage(dir: PathBuf) -> (u64, u64) {
    let disks = sysinfo::Disks::default();
    let mut av = 0u64;
    let mut to = 0u64;
    for d in disks.list() {
        if d.mount_point() == dir {
            av+=d.available_space();
            to+=d.total_space();
        }
        
    }
    return (av, to);
}
/// you enter the pathbuf of a dir and outputs all the contents 
/// of it and the size
fn get_files_in_storage(path: &PathBuf) -> (Vec<PathBuf>, u64) {
    let mut paths: Vec<PathBuf> = vec![];
    let mut storage = 0u64;
        
    let meta_dir = path
        .read_dir()
        .unwrap();
    for entry in meta_dir{
        let e = entry.unwrap();
        let path = e.path();
        let size = e.metadata().unwrap().len();
        paths.push(path);
        storage+=size
    }


    return (paths, storage);
}
