pub fn get_sql() -> String {
    let sql = r#"
CREATE TABLE IF NOT EXISTS user_jwts(
  id SERIAL PRIMARY KEY,
  host TEXT,
  date timestamp [ (p) ] [ without time zone ]	
);
"#
    .to_string();
    return sql;
}
