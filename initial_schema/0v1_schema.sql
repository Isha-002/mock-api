
DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'weekday') THEN
    CREATE TYPE weekday AS ENUM (
      'شنبه', 'یکشنبه', 'دوشنبه', 'سه شنبه', 'چهارشنبه', 'پنجشنبه', 'جمعه'
    );
  END IF;
END$$;

DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'role') THEN
    CREATE TYPE role AS ENUM (
      'customer',
      'restaurant_owner',
      'banned_user',
      'admin'
    );
  END IF;
END$$;



CREATE TABLE IF NOT EXISTS account (
  id UUID PRIMARY KEY,
  name TEXT NOT NULL,
  email TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL,
  phone_number TEXT NOT NULL UNIQUE,
  role role NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_account_phone_number ON account (phone_number);

CREATE TABLE IF NOT EXISTS restaurant (
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

CREATE TABLE IF NOT EXISTS owner (
  restaurant_id INT NOT NULL REFERENCES restaurant(id) ON DELETE CASCADE,
  account_id UUID REFERENCES account(id) ON DELETE CASCADE,
  national_id TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS food (
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

CREATE TABLE IF NOT EXISTS comments (
  id SERIAL,
  restaurant_id INT NOT NULL REFERENCES restaurant(id) ON DELETE CASCADE,
  account_id UUID REFERENCES account(id) ON DELETE CASCADE,
  text TEXT NOT NULL,
  rating INT NOT NULL DEFAULT 5 CHECK (rating >= 1 AND rating <= 5),
  created_on TIMESTAMP NOT NULL DEFAULT NOW(),
  PRIMARY KEY (id),
  UNIQUE (restaurant_id, account_id)
);
CREATE INDEX IF NOT EXISTS idx_comments_restaurant ON comments(restaurant_id);

CREATE TABLE IF NOT EXISTS comment_votes (
  account_id UUID NOT NULL REFERENCES account(id) ON DELETE CASCADE,
  comment_id INT NOT NULL REFERENCES comments(id) ON DELETE CASCADE,
  vote_type INT NOT NULL CHECK (vote_type IN (1, -1)),
  PRIMARY KEY (account_id, comment_id)
);

CREATE TABLE IF NOT EXISTS restaurant_hours (
  restaurant_id INT NOT NULL REFERENCES restaurant(id) ON DELETE CASCADE,
  day_of_week weekday NOT NULL,
  open_time TIME NOT NULL,
  close_time TIME NOT NULL,
  PRIMARY KEY (restaurant_id, day_of_week)
);



CREATE TABLE IF NOT EXISTS orders (
  id SERIAL PRIMARY KEY,
  account_id UUID NOT NULL REFERENCES account(id) ON DELETE CASCADE,
  status TEXT NOT NULL CHECK (status IN ('cart', 'pending', 'completed', 'canceled')),
  total_price INT NOT NULL,
  total_discounted_price INT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS orders_account_idx ON orders(account_id);
CREATE INDEX IF NOT EXISTS orders_created_idx ON orders(created_at);


CREATE TABLE IF NOT EXISTS item (
    id SERIAL PRIMARY KEY,
    order_id INT NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    account_id UUID NOT NULL REFERENCES account(id) ON DELETE CASCADE,
    restaurant_id INT NOT NULL REFERENCES restaurant(id) ON DELETE CASCADE,
    food_id INT NOT NULL REFERENCES food(id) ON DELETE CASCADE,
    quantity INT NOT NULL,
    name TEXT NOT NULL,
    image TEXT NOT NULL,
    price INTEGER NOT NULL,
    discount_price INTEGER,
    UNIQUE(order_id, food_id)
);



CREATE TABLE IF NOT EXISTS payment(
  id SERIAL PRIMARY KEY,
  account_id UUID NOT NULL REFERENCES account(id) ON DELETE CASCADE,
  restaurant_id INT NOT NULL REFERENCES restaurant(id),
  order_id INT NOT NULL REFERENCES orders(id),
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

CREATE INDEX IF NOT EXISTS idx_payment_order ON payment(order_id);
