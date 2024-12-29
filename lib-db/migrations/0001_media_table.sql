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
