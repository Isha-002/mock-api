
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

-- restaurant
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

-- owner
CREATE TABLE IF NOT EXISTS owner (
  restaurant_id INT NOT NULL REFERENCES restaurant(id) ON DELETE CASCADE,
  id UUID PRIMARY KEY,
  name TEXT,
  phone_number TEXT,
  email TEXT,
  national_id INT
)

--food
CREATE TABLE IF NOT EXISTS food (
  restaurant_id INT NOT NULL REFERENCES restaurant(id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  image TEXT NOT NULL,
  tag TEXT NOT NULL,
  price INT NOT NULL,
  discount BOOLEAN NOT NULL,
  discount_price INT,
  ingredient TEXT[] NOT NULL,
  available BOOLEAN NOT NULL
)

-- comments
CREATE TABLE IF NOT EXISTS comments (
  id SERIAL PRIMARY KEY,
  restaurant_id INT NOT NULL REFERENCES restaurant(id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  text TEXT NOT NULL,
  likes INT NOT NULL DEFAULT 0,
  dislikes INT NOT NULL DEFAULT 0,
  created_on TIMESTAMP NOT NULL DEFAULT NOW()
);

-- open/close time
CREATE TABLE IF NOT EXISTS restaurant_hours (
    id SERIAL PRIMARY KEY,
    restaurant_id INT NOT NULL REFERENCES restaurant(id) ON DELETE CASCADE,
    day_of_week VARCHAR(10),
    open_time TIME,
    close_time TIME
);


-- auth
CREATE TYPE role AS ENUM (
  'customer',
  'restaurant_owner',
  'banned_user',
  'admin'
);

CREATE TABLE account (
  id UUID PRIMARY KEY,        
  email TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL,
  phone_number TEXT NOT NULL UNIQUE,
  role role NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_account_phone_number ON account (phone_number);

307