pub mod ld_openssl;
pub mod ld_surrealdb;
pub mod ld_ffmpeg;
pub mod ld_nix;
pub fn depedncy_fn_check() {
    let path = "~/Connie";
    let open_check_test = ld_openssl::check(path);
    assert_eq!(open_check_test, 0);
    let surrealdb_check_test = ld_surrealdb::check(path);
    assert_eq!(surrealdb_check_test, 0);
}
#[cfg(test)]
mod tests {
    use super::*;
    fn depedncy_fn_check() {
        let path = "~/Connie";
        let open_check_test = ld_openssl::check(path);
        assert_eq!(open_check_test, 0);
        let surrealdb_check_test = ld_surrealdb::check(path);
        assert_eq!(surrealdb_check_test, 0);
    }
}