pub fn get_sql() -> String {
    let sql = r#"
CREATE TABLE IF NOT EXISTS "user"(
  id SERIAL PRIMARY KEY,
  cpid TEXT UNIQUE NOT NULL,
  name VARCHAR(50),
  username VARCHAR(50),
  host TEXT,
  email TEXT,
  password TEXT,
  CONSTRAINT fk_host 
    FOREIGN KEY(
      host
    ) REFERENCES server(cpid)
);
"#
    .to_string();

    return sql;
}
