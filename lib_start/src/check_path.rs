use std::{fs::{exists, File}, io::Result};

pub fn check_first_time() -> Result<u8> {
    let state: u8;
    let path: Vec<&str> = vec![
        "/opt/Connie/conf/db_conn",
        "/opt/Connie/conf/cie_ident.yaml",
        "/opt/Connie/conf/cie_config.yaml",

    ];
    let mut tmp: Vec<bool> = Vec::with_capacity(4);
    for p in path {
        let exist = exists(p)?;
        if exist {
            let f = File::open(p)?;
            let data = f.metadata()?;
            let len = data.len();
            if len > 0 {
                tmp.push(true);
            }
            else {
                tmp.push(false);
            }
        }
    }
    if tmp.contains(&false) {
        state = 1;
    }else {
        state = 0;
    }




    Ok(state)
}
