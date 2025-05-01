
DROP TABLE IF EXISTS comments CASCADE;
DROP TABLE IF EXISTS restaurant CASCADE;
DROP TYPE IF EXISTS foodType CASCADE;
DROP TYPE IF EXISTS role CASCADE;
DROP TABLE IF EXISTS account CASCADE;

CREATE TYPE foodType AS (
  name TEXT,
  price INT,
  ingredient TEXT[],
  available BOOLEAN
);


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

-- auth
CREATE TYPE role AS ENUM (
  'customer',
  'restaurant_owner',
  'banned_user',
  'admin'
);

CREATE TABLE account (
  id TEXT PRIMARY KEY,        
  email TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL,
  phone_number TEXT NOT NULL,
  role role NOT NULL
);

