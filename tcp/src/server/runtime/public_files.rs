use std::{io, path::PathBuf, str::FromStr};

use common_lib::{log::{debug, info},sha256::try_async_digest};
use lib_db::{
    media::media::{delete_media, Media},
    types::{sqlE, PgPool}
};

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


fn get_path(
    media_vec: &Vec<Media>
) -> Vec<PathBuf> {
    let mut result: Vec<PathBuf> = vec![];
    for m in media_vec {
        let path = PathBuf::from_str(&m.path);
        result.push(path.unwrap());
    };

    return result;
}

/// takes all of the db files then removes and deletes all non existent files and returns it 
async fn remove_deleted_file_records(
    mut media_vec: Vec<Media>,
    pool: &PgPool
) -> Result<(Vec<Media>, usize), sqlE> {
    let mut removed: Vec<usize>= vec![];
    let mut i = 0usize;
    for m in &media_vec {
        let path = PathBuf::from_str(&m.path).unwrap();
        if !path.exists() {
            removed.push(i); 
        };
        i+=1usize;
    }

    let number_of_elements = removed.len(); 
    i = 0usize; 
    loop {
        let index = number_of_elements - i;
        let m = media_vec.remove(index);
        delete_media(m, pool).await?;
        if i == number_of_elements {
            break;
        }
        i+=1
    }
    Ok((media_vec, number_of_elements))
}

fn get_new_files(
    media_vec: &Vec<Media>,
    dir: &PathBuf
) -> io::Result<(Vec<PathBuf>, usize)> {
    let path_vec = get_path(media_vec);
    let (files , ..) = get_files_in_storage(dir);
    let mut new_files: Vec<PathBuf> = vec![];
    let mut nfiles_added = 0usize;
    if files.is_empty(){
        return Ok((new_files, nfiles_added));
    };
    if !path_vec.is_empty() {
        for f in files {
            if !path_vec.contains(&f) {
                new_files.push(f);
                nfiles_added+=1;
            }
        };
    } else {
        let len = files.len();
        return Ok((files, len)); 
    }

    Ok((new_files, nfiles_added))
}

async fn create_new_records(
    new_files: Vec<PathBuf>,
    host_cpid: &String,
    pool: &PgPool
) -> Result<(), sqlE> {
    
    assert!(new_files.len() != 0usize);
    for f in new_files {
        let name = f.file_name().unwrap()
            .to_str().unwrap().to_owned();
        let path = f.to_str().unwrap().to_owned();
        let type_ = f.extension().unwrap()
            .to_str().unwrap().to_owned();
        let metadata = f.metadata().unwrap();
        let size = metadata.len() as i64;
        
        debug!("creating checksum for {:?}", f);
        let checksum = try_async_digest(&f)
            .await
            .unwrap();
        debug!("created checksum for {:?}", f);

        let media = Media {
            name,
            size,
            in_host: host_cpid.to_owned(),
            cpid: host_cpid.to_owned(),
            path,
            checksum,
            type_
        };
        let _res = media.post(pool).await?;
        assert!(_res == 0);
        debug!("pub file added to db succesfully");
    }
    
    Ok(())
}



pub async fn new_pub_files_process(
    dir: &str,
    host_cpid: &String,
    pool: &PgPool,
) -> Result<(), sqlE> {
    
    let dir = PathBuf::from_str(dir).unwrap();
    if !dir.is_dir() || !dir.exists() {
        return Ok(());
    }
    let (files , ..) = get_files_in_storage(&dir);
    create_new_records(files, host_cpid, pool).await?;
    Ok(())
}


pub async fn pub_files_process(
    db_files: Vec<Media>,
    dir: &str,
    host_cpid: &String,
    pool: &PgPool,
) -> Result<(), sqlE> {
    let dir = PathBuf::from_str(dir).unwrap();
    if !dir.is_dir() || !dir.exists() {
        return Ok(());
    }
    let (cleaned_vec, nfiles_removed) = remove_deleted_file_records(db_files, pool).await?;
    if nfiles_removed != 0usize {
        info!("pub files removed: {}", nfiles_removed);
    };
    
    let (new_files, nfiles_added) = get_new_files(&cleaned_vec, &dir).unwrap();
    if nfiles_added != 0usize {
        info!("pub files added: {}", nfiles_added);
    }
    create_new_records(new_files, host_cpid, pool).await?;
    Ok(())
}


