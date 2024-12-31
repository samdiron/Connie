fn get_sql() -> String {
    let sql = r#"
    CREATE TABLE admin(
    id SERIAL PRIMARY KEY,
    cpid TEXT PRIMARY KEY
    );
    "#
    .to_string();
    return sql;
}
