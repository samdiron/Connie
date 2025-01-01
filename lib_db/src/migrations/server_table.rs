pub fn get_sql() -> String {
    let sql = r#"
CREATE TABLE IF NOT EXISTS server(
  id SERIAL PRIMARY KEY,
  cpid TEXT UNIQUE NOT NULL,
  name VARCHAR(50),
  host TEXT,
  memory INT,
  storage INT,
  max_conn INT,
  password TEXT
);
"#
    .to_string();
    return sql;
}
