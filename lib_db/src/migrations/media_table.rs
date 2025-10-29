pub fn get_sql() -> String {
    let sql = r#"
CREATE TABLE IF NOT EXISTS media(
  id SERIAL PRIMARY KEY,
  cpid TEXT, 
  name TEXT,
  in_host TEXT,
  "type" VARCHAR(10),
  checksum TEXT NOT NULL,
  "size" BIGINT,
  -- removing the cpid constraint because the OR keyword is not valid on all pg_version 
  -- CONSTRAINT fk_cpid 
  --   FOREIGN KEY(
  --     cpid
  --   ) REFERENCES "user"(cpid) OR server(cpid),
  CONSTRAINT fk_host 
    FOREIGN KEY(
      in_host
    ) REFERENCES server(cpid),
  path TEXT
);
"#
    .to_string();

    return sql;
}
