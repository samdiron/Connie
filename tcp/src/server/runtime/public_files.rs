use std::path::PathBuf;

// use common_lib::path::PUBLIC_DATA_DIR;
// use lib_db::media::media::Media;


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
//
// async fn check_public_files_from_storage(
//     db_files: Vec<Media>
// ) -> Result<(Vec<PathBuf>)> {
//     let path = PathBuf::from_str(PUBLIC_DATA_DIR).unwrap();
//     let (files, size) = get_files_in_storage(&path);
//     for df in db_files {
//         if !files.contains(&df) {
//             sqlite_delete_media(
//                 &host,
//                 &cpid,
//                 &df.checksum,
//                 pool
//             ).await;
//             deleted+=1;
//         };
//     }
//
// }





