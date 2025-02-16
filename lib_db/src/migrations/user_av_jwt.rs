pub fn get_sql() -> String {
    let sql = r#"
CREATE TABLE IF NOT EXISTS user_jwt(
  id SERIAL PRIMARY KEY,
  host TEXT,
  cpid TEXT ,
  jwt TEXT,
  date BIGINT,
  CONSTRAINT fk_jwt 
    FOREIGN KEY(cpid)
      REFERENCES "user"(cpid)
  
);
"#
    .to_string();
    return sql;
}
