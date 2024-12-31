pub fn get_sql() -> String {
    let sql = r#"
CREATE TABLE IF NOT EXIST "user"(
  id SERIAL PRIMARY KEY,
  cpid TEXT PRIMARY KEY,
  name VARCHAR(50),
  username VARCHAR(50),
  host VARCHAR(50),
  email TEXT,
  CONSTRAINT fk_host FOREIGN KEY (host)
    REFERENCES server(cpid)
  password TEXT,
);
"#
    .to_string();

    return sql;
}
