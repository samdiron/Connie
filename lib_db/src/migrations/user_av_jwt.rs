pub fn get_sql() -> String {
    let sql = r#"
CREATE TABLE IF NOT EXISTS user_jwt(
  id SERIAL PRIMARY KEY,
  host TEXT,
  jwt TEXT,
  date BIGINT
);
"#
    .to_string();
    return sql;
}
