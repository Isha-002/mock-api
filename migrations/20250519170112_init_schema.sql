
DROP TABLE IF EXISTS comments CASCADE;
DROP TABLE IF EXISTS food CASCADE;
DROP TABLE IF EXISTS comment_votes CASCADE;
DROP TABLE IF EXISTS owner CASCADE;
DROP TABLE IF EXISTS orders_active CASCADE;
DROP TABLE IF EXISTS orders_archived CASCADE;
DROP TABLE IF EXISTS orders CASCADE;
DROP TABLE IF EXISTS payment CASCADE;
DROP TABLE IF EXISTS restaurant_hours CASCADE;
DROP TABLE IF EXISTS restaurant CASCADE;
DROP TABLE IF EXISTS account CASCADE;
DROP TYPE IF EXISTS weekday CASCADE;
DROP TYPE IF EXISTS role CASCADE;

-- Enum types
CREATE TYPE weekday AS ENUM (
  'شنبه', 'یکشنبه', 'دوشنبه', 'سه شنبه', 'چهارشنبه', 'پنجشنبه', 'جمعه'
);

CREATE TYPE role AS ENUM (
  'customer',
  'restaurant_owner',
  'banned_user',
  'admin'
);


CREATE TABLE account (
  id UUID PRIMARY KEY,
  name TEXT NOT NULL,
  email TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL,
  phone_number TEXT NOT NULL UNIQUE,
  role role NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_account_phone_number ON account (phone_number);

CREATE TABLE restaurant (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  rating REAL NOT NULL DEFAULT 0,
  distance DOUBLE PRECISION NOT NULL,
  tags TEXT[],
  image TEXT NOT NULL,
  address TEXT NOT NULL,
  city TEXT NOT NULL,
  location DOUBLE PRECISION[2],
  created_on TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE owner (
  restaurant_id INT NOT NULL REFERENCES restaurant(id) ON DELETE CASCADE,
  account_id UUID REFERENCES account(id) ON DELETE CASCADE,
  name TEXT,
  phone_number TEXT,
  email TEXT,
  national_id INT
);

CREATE TABLE food (
  id SERIAL PRIMARY KEY,
  restaurant_id INT NOT NULL REFERENCES restaurant(id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  image TEXT NOT NULL,
  tag TEXT NOT NULL,
  price INT NOT NULL,
  discount BOOLEAN NOT NULL,
  discount_price INT,
  ingredient TEXT[] NOT NULL,
  available BOOLEAN NOT NULL,
  CONSTRAINT discount_price_required 
    CHECK (NOT discount OR (discount AND discount_price IS NOT NULL))
);

CREATE TABLE comments (
  id SERIAL,
  restaurant_id INT NOT NULL REFERENCES restaurant(id) ON DELETE CASCADE,
  account_id UUID REFERENCES account(id) ON DELETE CASCADE,
  text TEXT NOT NULL,
  rating INT NOT NULL DEFAULT 5 CHECK (rating >= 1 AND rating <= 5),
  created_on TIMESTAMP NOT NULL DEFAULT NOW(),
  PRIMARY KEY (id),
  UNIQUE (restaurant_id, account_id)
);
CREATE INDEX idx_comments_restaurant ON comments(restaurant_id);

CREATE TABLE comment_votes (
  account_id UUID NOT NULL REFERENCES account(id) ON DELETE CASCADE,
  comment_id INT NOT NULL REFERENCES comments(id) ON DELETE CASCADE,
  vote_type INT NOT NULL CHECK (vote_type IN (1, -1)),
  PRIMARY KEY (account_id, comment_id)
);

CREATE TABLE restaurant_hours (
  restaurant_id INT NOT NULL REFERENCES restaurant(id) ON DELETE CASCADE,
  day_of_week weekday NOT NULL,
  open_time TIME NOT NULL,
  close_time TIME NOT NULL,
  PRIMARY KEY (restaurant_id, day_of_week)
);

-- Partitioned orders table
CREATE TABLE orders (
  id SERIAL,
  account_id UUID NOT NULL REFERENCES account(id) ON DELETE CASCADE,
  restaurant_id INT NOT NULL REFERENCES restaurant(id) ON DELETE CASCADE,
  food_names TEXT[] NOT NULL,
  status TEXT NOT NULL DEFAULT 'cart' CHECK (status IN ('cart', 'pending', 'completed', 'canceled')),
  total_price INT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  PRIMARY KEY (id, status)
) PARTITION BY LIST(status);

CREATE TABLE orders_active PARTITION OF orders
  FOR VALUES IN ('cart', 'pending');

CREATE TABLE orders_archived PARTITION OF orders
  FOR VALUES IN ('completed', 'canceled');

CREATE INDEX orders_account_idx ON orders(account_id);
CREATE INDEX orders_restaurant_idx ON orders(restaurant_id);
CREATE INDEX orders_created_idx ON orders(created_at);

CREATE TABLE payment (
  id SERIAL PRIMARY KEY,
  account_id UUID NOT NULL REFERENCES account(id) ON DELETE CASCADE,
  restaurant_id INT NOT NULL REFERENCES restaurant(id),
  order_id INT NOT NULL,
  order_status TEXT NOT NULL,
  FOREIGN KEY (order_id, order_status) REFERENCES orders(id, status),
  cash BOOLEAN NOT NULL,
  paid_money INT,
  transaction_id TEXT,
  card_number TEXT,
  CONSTRAINT paid_with_cash
    CHECK (cash OR (
      paid_money IS NOT NULL AND
      transaction_id IS NOT NULL AND
      card_number IS NOT NULL
    ))
);

