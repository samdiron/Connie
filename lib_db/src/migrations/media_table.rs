pub fn get_sql() -> String {
    let sql = r#"
CREATE TABLE IF NOT EXISTS media(
  id SERIAL PRIMARY KEY,
  cpid TEXT, 
  name TEXT,
  in_host TEXT,
  "type" VARCHAR(10),
  checksum TEXT,
  "size" BIGINT,
  CONSTRAINT fk_cpid 
    FOREIGN KEY(
      cpid
    ) REFERENCES "user"(cpid),
  path TEXT
);
"#
    .to_string();

    return sql;
}
