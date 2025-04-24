
DROP TABLE IF EXISTS comments CASCADE;
DROP TABLE IF EXISTS restaurant CASCADE;
DROP TYPE IF EXISTS foodType CASCADE;
DROP TYPE IF EXISTS commentType CASCADE;

CREATE TYPE foodType AS (
  name TEXT,
  price INT,
  ingredient TEXT[],
  available BOOLEAN
);

-- CREATE TYPE commentType AS (
--   name TEXT,
--   text TEXT,
--   likes INT,
--   dislikes INT
-- );

CREATE TABLE IF NOT EXISTS restaurant (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  rating REAL NOT NULL,
  distance DOUBLE PRECISION NOT NULL,
  tags TEXT[],
  menu foodType[] NOT NULL,
  image TEXT NOT NULL,
  city TEXT NOT NULL,
  address TEXT NOT NULL,
  created_on TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS comments (
  id SERIAL PRIMARY KEY,
  restaurant_id INT NOT NULL REFERENCES restaurant(id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  text TEXT NOT NULL,
  likes INT NOT NULL DEFAULT 0,
  dislikes INT NOT NULL DEFAULT 0,
  created_on TIMESTAMP NOT NULL DEFAULT NOW()
);

