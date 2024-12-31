pub fn get_sql() -> String {
    let sql = r#"
CREATE TABLE IF NOT EXISTS media(
  id SERIAL PRIMARY KEY,
  cpid TEXT, 
  name TEXT,
  "type" VARCHAR(10),
  "size" INT,
  CONSTRAINT fk_cpid 
    FOREIGN KEY(
      cpid
    ) REFERENCES "user"(cpid)
  "path" TEXT
  
);
"#
    .to_string();

    return sql;
}
