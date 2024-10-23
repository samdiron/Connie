use std::path::PathBuf;
use directories::{
    BaseDirs,
    ProjectDirs,
};

pub fn c_path () -> String {
    let mut path :PathBuf = PathBuf::new();
    if let Some(project_dir) = ProjectDirs::from("com", "https://github.com/samdiron/Connie" ,"Connie"){
        *&mut path = project_dir.config_dir().to_owned();
    }
    let sg = path.to_str().unwrap();
    let string = sg.to_owned();
    return string
}
pub fn h_path() -> String {
    let mut path :PathBuf = PathBuf::new();
    if let Some(base) = BaseDirs::new() {
        *&mut path = base.home_dir().to_owned();
    }
    let sg = path.to_str().unwrap();
    let string = sg.to_owned();
    return string
}
