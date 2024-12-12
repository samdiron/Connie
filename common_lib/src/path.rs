use directories::{BaseDirs, ProjectDirs};
use std::path::PathBuf;
// use std::{fmt::format, fs::File};

pub fn config_path() -> String {
    let mut path: PathBuf = PathBuf::new();
    if let Some(project_dir) =
        ProjectDirs::from("com", "https://github.com/samdiron/Connie", "Connie")
    {
        *&mut path = project_dir.config_dir().to_owned();
    }
    let sg = path.to_str().unwrap();
    let string = sg.to_owned();
    return string;
}
pub fn get_home_path() -> String {
    let mut path: PathBuf = PathBuf::new();
    if let Some(base) = BaseDirs::new() {
        *&mut path = base.home_dir().to_owned();
    }
    let sg = path.to_str().unwrap();
    let string = sg.to_owned();
    let string = format!("{string}/Connie");
    return string;
}

// struct Path {
//     name: String,
//     main: bool,
//     root: bool,
//     config: bool
// }
//
// impl Path {
//     fn get_checked(Self { name, main, root, config }: Self) -> File {
//         let path
//     }
// }
//
