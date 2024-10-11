pub mod ld_openssl;
mod ld_surrealdb;
mod ld_ffmpeg;
mod ld_nix;

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