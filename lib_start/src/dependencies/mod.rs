pub mod ld_ffmpeg;
pub mod ld_nix;
pub mod ld_openssl;
pub mod ld_surrealdb;



//
//
// pub fn depedncy_fn_check() {
//     let path = "~/Connie";
//     let open_check_test = ld_openssl::openssl_ld_check(path);
//     assert_eq!(open_check_test, 0);
//     let surrealdb_check_test = surreal_ld_check(path);
//     assert_eq!(surrealdb_check_test, 0);
// }
// #[cfg(test)]
// mod tests {
//     use ld_surrealdb::surreal_ld_check;
//
//     use super::*;
//     fn depedncy_fn_check() {
//         let path = "~/Connie";
//         let open_check_test = ld_openssl::openssl_ld_check(path);
//         assert_eq!(open_check_test, 0);
//         let surrealdb_check_test = surreal_ld_check(path);
//         assert_eq!(surrealdb_check_test, 0);
//     }
// }
//
