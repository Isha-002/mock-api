DROP TABLE IF EXISTS comments CASCADE;
DROP TABLE IF EXISTS restaurant CASCADE;
DROP TYPE IF EXISTS foodType CASCADE;
DROP TYPE IF EXISTS role CASCADE;
DROP TABLE IF EXISTS account CASCADE;


-- restaurant -----------------------------------
-- get restaurants with limit:
----------------------------------------------------------
-- SELECT * from restaurant LIMIT $1 OFFSET $2
----------------------------------------------------------
-- post a restaurant (role check handled in application layer):
----------------------------------------------------------
-- INSERT INTO restaurant (name, rating, distance, tags, image, address, city, location)
-- VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
-- RETURNING id, name, rating, distance, tags, image, address, city, location
----------------------------------------------------------
-- update a restaurant (role check handled in application layer):
----------------------------------------------------------
-- UPDATE restaurant 
-- SET name = $1, rating = $2, distance = $3, tags = $4, image = $5, address = $6, city = $7, location = $8
-- WHERE id = $9
-- RETURNING id, name, rating, distance, tags, image, address, city, location
----------------------------------------------------------
-- delete a restaurant (role check handled in application layer):
----------------------------------------------------------
-- DELETE FROM restaurant WHERE id = $1
----------------------------------------------------------
-- update restaurant image (role check handled in application layer):
----------------------------------------------------------
-- UPDATE restaurant 
-- SET image = $1
-- WHERE id = $2
----------------------------------------------------------
-- get by city:
----------------------------------------------------------
-- SELECT * from restaurant
-- WHERE city = $1
-- ORDER BY rating DESC
----------------------------------------------------------
-- get by name and city:
----------------------------------------------------------
-- SELECT * from restaurant
-- WHERE name = $1 AND city = $2
-- ORDER BY rating DESC
----------------------------------------------------------
-- get by tag and city
-- SELECT * from restaurant
-- WHERE $1 = ANY(tags) AND city = $2
-- ORDER BY rating DESC
----------------------------------------------------------
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
-- ////////////////////////////////////////////////////////////////
-- owner -----------------------------------
-- get owner:
----------------------------------------------------------
-- SELECT name, phone_number, email, national_id
-- FROM owner
-- WHERE restaurant_id = $1
-- AND account_id = $2
----------------------------------------------------------
-- post owner (role check handled in application layer):
----------------------------------------------------------
-- INSERT INTO owner (restaurant_id, account_id, name, phone_number, email, national_id)
-- VALUES ($1, $2, $3, $4, $5, $6)
-- RETURNING *;
----------------------------------------------------------
-- put owner (role check handled in application layer):
----------------------------------------------------------
-- UPDATE owner 
-- SET name= $1, phone_number= $2, email= $3, national_id= $4
-- WHERE restaurant_id = $5
-- RETURNING *;
----------------------------------------------------------
-- delete owner (admin only - handled in application layer):
----------------------------------------------------------
-- WITH deleted_owner AS (
--   DELETE FROM owner WHERE account_id = $1
--   RETURNING restaurant_id
-- )
-- DELETE FROM restaurant
-- WHERE id IN (SELECT restaurant_id FROM deleted_owner);
----------------------------------------------------------
-- transfer ownership (admin only - handled in application layer):
----------------------------------------------------------
-- UPDATE owner 
-- SET account_id= $1, name= $2, phone_number= $3, email= $4, national_id= $5
-- WHERE restaurant_id = $6
-- RETURNING *;
----------------------------------------------------------
CREATE TABLE IF NOT EXISTS owner (
  restaurant_id INT NOT NULL REFERENCES restaurant(id) ON DELETE CASCADE,
  account_id UUID REFERENCES account(id) ON DELETE CASCADE,
  name TEXT,
  phone_number TEXT,
  email TEXT,
  national_id INT
);
-- ////////////////////////////////////////////////////////////////
-- food -----------------------------------
-- get the menu:
----------------------------------------------------------
-- SELECT id, name, image, tag, price, discount, discount_price, ingredient, available
-- FROM food 
-- WHERE restaurant_id = $1;
----------------------------------------------------------
-- post new food:
----------------------------------------------------------
-- INSERT INTO food (restaurant_id, name, image, tag, price, discount, discount_price, ingredient, available)
-- VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
-- RETURNING *;
----------------------------------------------------------
-- get one food(by name):
----------------------------------------------------------
-- SELECT id ,name, image, tag, price, discount, discount_price, ingredient, available
-- FROM food 
-- WHERE restaurant_id = $1
-- AND name = $2;
----------------------------------------------------------
-- put food (role check handled in application layer):
----------------------------------------------------------
-- UPDATE food
-- SET name= $1, image= $2, tag= $3, price= $4, discount= $5, discount_price= $6,
-- ingredient= $7, available= $8
-- WHERE restaurant_id = $9
-- AND name = $10
-- RETURNING *;
----------------------------------------------------------
-- delete food(single) (role check handled in application layer): 
----------------------------------------------------------
-- DELETE FROM food
-- WHERE restaurant_id = $1
-- AND id = $2
----------------------------------------------------------
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

CREATE INDEX idx_food_restaurant ON food(restaurant_id);
CREATE INDEX idx_food_name ON food(name);
-- ////////////////////////////////////////////////////////////////
-- comments -----------------------------------
-- get comments + votes + current user vote query:
----------------------------------------------------------
-- SELECT
--     comments.id,
--     comments.text,
--     comments.rating,
--     comments.created_on,
--     account.name,
--     COUNT(CASE WHEN comment_votes.vote_type = 1 THEN 1 END) AS likes,
--     COUNT(CASE WHEN comment_votes.vote_type = -1 THEN 1 END) AS dislikes,
--     MAX(CASE WHEN comment_votes.account_id = $1 THEN comment_votes.vote_type END) AS current_user_vote
-- FROM comments
-- LEFT JOIN account ON comments.account_id = account.id 
-- LEFT JOIN comment_votes ON comments.id = comment_votes.comment_id 
-- WHERE comments.restaurant_id = $2
-- GROUP BY comments.id, comments.text, comments.rating, comments.created_on, account.name;
----------------------------------------------------------
-- post comments query:
----------------------------------------------------------
-- INSERT INTO comments (restaurant_id, account_id, text, rating)
-- VALUES ($1, $2, $3, $4)
-- ON CONFLICT (restaurant_id, account_id) DO NOTHING
-- RETURNING *;
----------------------------------------------------------
-- delete a comment:
----------------------------------------------------------
-- DELETE FROM comments
-- WHERE id = $1
-- AND (account_id = $2 OR EXISTS (
--   SELECT 1 FROM account WHERE id = $2 AND role = 'admin'
-- ));
----------------------------------------------------------
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

CREATE INDEX idx_comments_restaurant ON comments(restaurant_id);


-- ////////////////////////////////////////////////////////////////
-- like/dislike -----------------------------------
-- vote query:
----------------------------------------------------------
-- INSERT INTO comment_votes (account_id, comment_id, vote_type)
-- VALUES ($1, $2, $3)
-- ON CONFLICT (account_id, comment_id)
-- DO UPDATE SET vote_type = EXCLUDED.vote_type;
----------------------------------------------------------
-- remove vote query:
----------------------------------------------------------
-- DELETE FROM comment_votes
-- WHERE account_id = $1
-- AND comment_id = $2;
----------------------------------------------------------
CREATE TABLE IF NOT EXISTS comment_votes (
  account_id UUID NOT NULL REFERENCES account(id) ON DELETE CASCADE,
  comment_id INT NOT NULL REFERENCES comments(id) ON DELETE CASCADE,
  vote_type INT NOT NULL CHECK (vote_type IN (1, -1)),
  PRIMARY KEY (account_id, comment_id)
);

-- ////////////////////////////////////////////////////////////////
-- open/close time -----------------------------------
-- get all days:
----------------------------------------------------------
-- SELECT day_of_week, open_time, close_time FROM restaurant_hours
-- WHERE restaurant_id = $1
-- ORDER BY day_of_week; 
----------------------------------------------------------
-- post a day (role check handled in application layer):
----------------------------------------------------------
-- INSERT INTO restaurant_hours (restaurant_id, day_of_week, open_time, close_time)
-- VALUES ($1, $2, $3, $4);
----------------------------------------------------------
-- put a day (role check handled in application layer):
----------------------------------------------------------
-- UPDATE restaurant_hours
-- SET open_time = $1, close_time = $2
-- WHERE restaurant_id = $3
-- AND day_of_week = $4;
----------------------------------------------------------
-- delete a day (role check handled in application layer):
----------------------------------------------------------
-- DELETE FROM restaurant_hours 
-- WHERE restaurant_id = $1
-- AND day_of_week = $2
-- RETURNING *;
----------------------------------------------------------
CREATE TYPE weekday AS ENUM (
  'شنبه',       
  'یکشنبه',     
  'دوشنبه',     
  'سه شنبه',    
  'چهارشنبه',    
  'پنجشنبه',     
  'جمعه'       
);

CREATE TABLE IF NOT EXISTS restaurant_hours (
  restaurant_id INT NOT NULL REFERENCES restaurant(id) ON DELETE CASCADE,
  day_of_week weekday NOT NULL,
  open_time TIME NOT NULL,
  close_time TIME NOT NULL,
  PRIMARY KEY (restaurant_id, day_of_week)
);

-- ////////////////////////////////////////////////////////////////
-- auth -----------------------------------
CREATE TYPE role AS ENUM (
  'customer',
  'restaurant_owner',
  'banned_user',
  'admin'
);

CREATE TABLE IF NOT EXISTS account (
  id UUID PRIMARY KEY,
  name TEXT NOT NULL,
  email TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL,
  phone_number TEXT NOT NULL UNIQUE,
  role role NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_account_phone_number ON account (phone_number);

-- ////////////////////////////////////////////////////////////////
-- order -----------------------------------
CREATE TABLE IF NOT EXISTS orders (
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

-- payment -----------------------------------
CREATE table IF NOT EXISTS payment(
  id SERIAL PRIMARY KEY,
  account_id UUID NOT NULL REFERENCES account(id) ON DELETE CASCADE,
  restaurant_id INT NOT NULL REFERENCES restaurant(id),
  --
  order_id INT NOT NULL,
  order_status TEXT NOT NULL,
  FOREIGN KEY (order_id, order_status) REFERENCES orders(id, status),
  --
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
)

CREATE INDEX idx_payment_order ON payment(order_id);


