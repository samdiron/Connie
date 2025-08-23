
use std::sync::{
    Mutex,
    LazyLock,
};
use crate::common::request::RQM;

// statistics 
pub static ALL_REQUESTS: LazyLock<Mutex<Vec<RQM>>> = LazyLock::new(||{
    let vector: Mutex<Vec<RQM>> = Mutex::new(vec![]);
    vector
});
pub static N_ALL_TIME_REQUESTS: LazyLock<Mutex<u64>> = LazyLock::new(||{
    let val = 0u64;
    Mutex::new(val)
});
pub static N_FAILED_REQUESTS: LazyLock<Mutex<u64>> = LazyLock::new(||{
    let val = 0u64;
    Mutex::new(val)
});
pub static N_SUCCESFUL_REQUESTS: LazyLock<Mutex<u64>> = LazyLock::new(||{
    let val = 0u64;
    Mutex::new(val)
});
pub static N_CURRENT_REQUESTS: LazyLock<Mutex<u64>> = LazyLock::new(||{
    let val = 0u64;
    Mutex::new(val)
});
pub static DOWN_BAND_WIDTH: LazyLock<Mutex<usize>> = LazyLock::new(||{
    let val = 0usize;
    Mutex::new(val)
});
pub static UP_BAND_WIDTH: LazyLock<Mutex<usize>> = LazyLock::new(||{
    let val = 0usize;
    Mutex::new(val)
});



// flow control
pub static ALLOW_NOTLS: LazyLock<Mutex<bool>> = LazyLock::new(||{
    let val = false;
    Mutex::new(val)
});
pub static STOP_LISTENING: LazyLock<Mutex<bool>> = LazyLock::new(||{
    let val = false;
    Mutex::new(val)
});
pub static QUIT: LazyLock<Mutex<bool>> = LazyLock::new(||{
    let val = false;
    Mutex::new(val)
});
pub static RESTART: LazyLock<Mutex<bool>> = LazyLock::new(||{
    let val = false;
    Mutex::new(val)
});
pub static GET_REQUESTS_ONLY: LazyLock<Mutex<bool>> = LazyLock::new(||{
    let val = false;
    Mutex::new(val)
});
pub static NO_DELETE_REQUESTS: LazyLock<Mutex<bool>> = LazyLock::new(||{
    let val = false;
    Mutex::new(val)
});

/// should be reverted right after the cleaning process starts
pub static START_CLEANING: LazyLock<Mutex<bool>> = LazyLock::new(||{
    let val = false;
    Mutex::new(val)
});
