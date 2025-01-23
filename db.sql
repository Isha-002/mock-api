CREATE TYPE food AS (
  name VARCHAR(100),
  price INT,
  ingredient TEXT[],
  available BOOLEAN
);


CREATE TABLE IF NOT EXISTS restaurant (
  id SERIAL PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  rating REAL NOT NULL,
  distance DOUBLE PRECISION NOT NULL,
  tags TEXT[],
  menu food[] NOT NULL,
  image VARCHAR(255) NOT NULL,
  created_on TIMESTAMP NOT NULL DEFAULT NOW()
);
