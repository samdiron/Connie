use std::io::ErrorKind;
use lib_db::database::check_conn;



pub fn valid_db_conf() -> std::result::Result<(), ErrorKind> {
    let pool = check_conn();
    if pool == 1 {
        println!("invalied db connection string in db_conn file");
        let e = ErrorKind::InvalidInput;
        return Err(e)
    }

    Ok(())
}

