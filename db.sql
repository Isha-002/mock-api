-- 
-- DROP TYPE IF EXISTS food CASCADE;
-- 
CREATE TYPE food AS (
  name TEXT,
  price INT,
  ingredient TEXT[],
  available BOOLEAN
);
-- to modify types otherwise remove this line
-- DROP TABLE IF EXISTS restaurant CASCADE; 
-- 
CREATE TABLE IF NOT EXISTS restaurant (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  rating REAL NOT NULL,
  distance DOUBLE PRECISION NOT NULL,
  tags TEXT[],
  menu food[] NOT NULL,
  image TEXT NOT NULL,
  created_on TIMESTAMP NOT NULL DEFAULT NOW()
);
