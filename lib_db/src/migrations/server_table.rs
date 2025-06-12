pub fn get_sql() -> String {
    let sql = r#"
CREATE TABLE IF NOT EXISTS server(
  id SERIAL PRIMARY KEY,
  cpid TEXT UNIQUE NOT NULL,
  name VARCHAR(50),
  host TEXT,
  pri_ip TEXT,
  pub_ip TEXT,
  memory BIGINT,
  max_conn SMALLINT,
  password TEXT
);
"#
    .to_string();
    return sql;
}
