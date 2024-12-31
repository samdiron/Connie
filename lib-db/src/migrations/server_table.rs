pub fn get_sql() -> String {
    let sql = r#"
CREATE TABLE IF NOT EXIST server(
  id SERIAL PRIMARY KEY,
  cpid TEXT PRIMARY KEY,
  name VARCHAR(50),
  host TEXT,
  memory INT,
  workers INT,
  storage INT,
  max_conn INT,
  password TEXT,
);
"#
    .to_string();
    return sql;
}
