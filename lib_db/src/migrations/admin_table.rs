pub fn get_sql() -> String {
    let sql = r#"
    CREATE TABLE IF NOT EXISTS admin(
    id SERIAL PRIMARY KEY,
    cpid TEXT UNIQUE NOT NULL
    );
    "#
    .to_string();
    return sql;
}
