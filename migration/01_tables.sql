DROP TABLE IF EXISTS comments CASCADE;
DROP TABLE IF EXISTS restaurant CASCADE;
DROP TYPE IF EXISTS foodType CASCADE;
DROP TYPE IF EXISTS role CASCADE;
DROP TABLE IF EXISTS account CASCADE;


-- restaurant -----------------------------------
CREATE TABLE IF NOT EXISTS restaurant (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  rating REAL NOT NULL,
  distance DOUBLE PRECISION NOT NULL,
  tags TEXT[],
  image TEXT NOT NULL,
  city TEXT NOT NULL,
  address TEXT NOT NULL,
  location DOUBLE PRECISION[2],
  created_on TIMESTAMP NOT NULL DEFAULT NOW()
);
-- ////////////////////////////////////////////////////////////////
-- owner -----------------------------------
CREATE TABLE IF NOT EXISTS owner (
  restaurant_id INT NOT NULL REFERENCES restaurant(id) ON DELETE CASCADE,
  id UUID PRIMARY KEY,
  name TEXT,
  phone_number TEXT,
  email TEXT,
  national_id INT
)
-- ////////////////////////////////////////////////////////////////
-- food -----------------------------------
-- get the menu:
----------------------------------------------------------
-- SELECT name, image, tag, price, discount, discount_price, ingredient, available
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
-- SELECT name, image, tag, price, discount, discount_price, ingredient, available
-- FROM food 
-- WHERE restaurant_id = $1
-- AND name = $2;
----------------------------------------------------------
-- put food:
----------------------------------------------------------
-- UPDATE food
-- SET name= $1, image= $2, tag= $3, price= $4, discount= $5, discount_price= $6,
-- ingredient= $7, available= $8
-- WHERE restaurant_id = $9
-- AND name = $10
-- AND role = 'admin'
-- OR role = 'restaurant_owner'
-- RETURNING *;
----------------------------------------------------------
-- delete food(single): 
----------------------------------------------------------
-- DELETE FROM food
-- WHERE restaurant_id = $1
-- AND name = $2
-- AND role = 'admin'
-- OR role = 'restaurant_owner'
----------------------------------------------------------
CREATE TABLE IF NOT EXISTS food (
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
)
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
--     COUNT(CASE WHEN comment_votes.vote_type = 1 THEN 1 ELSE 0 END) AS likes,
--     COUNT(CASE WHEN comment_votes.vote_type = -1 THEN 1 ELSE 0 END) AS dislikes,
--     MAX(CASE WHEN comment_votes.account_id = current_user_id THEN comment_votes.vote_type ELSE NULL END) AS current_user_vote
-- FROM
--     comments
-- LEFT JOIN
--     account ON comments.account_id = account.id 
-- LEFT JOIN
--     comment_votes ON comments.id = comment_votes.comment_id 
-- WHERE
--     comments.restaurant_id = your_restaurant_id
-- GROUP BY
--     comments.id,
--     comments.text,
--     comments.rating,
--     comments.created_on,
--     account.name,
----------------------------------------------------------
-- post comments query(cant post more than 1):
----------------------------------------------------------
-- INSERT INTO comments (restaurant_id, account_id, text, rating)
-- VALUES ($1, $2, $3, $4)
-- ON CONFLICT(account_id) DO NOTHING
-- RETURNING *;
----------------------------------------------------------
-- delete a comment:
----------------------------------------------------------
-- DELETE FROM comments
-- WHERE id = comment_id 
--   AND (
--     account_id = id(from account table)  
--     OR 
--     EXISTS (         
--       SELECT 1 FROM accounts 
--       WHERE id = $2 AND role = 'admin'
--     )
--   )
----------------------------------------------------------
CREATE TABLE IF NOT EXISTS comments (
  id SERIAL,
  restaurant_id INT NOT NULL REFERENCES restaurant(id) ON DELETE CASCADE,
  account_id UUID REFERENCES account(id) ON DELETE CASCADE,
  text TEXT NOT NULL,
  rating INT NOT NULL DEFAULT 5 CHECK (rating >= 1 AND rating <= 5)
  created_on TIMESTAMP NOT NULL DEFAULT NOW()
  PRIMARY (id, account_id)
);

CREATE INDEX idx_comments_restaurant ON comments(restaurant_id);
CREATE INDEX idx_comment_votes_comment ON comment_votes(comment_id);

-- ////////////////////////////////////////////////////////////////
-- like/dislike -----------------------------------
-- vote query:
----------------------------------------------------------
-- INSERT INTO comment_votes (user_id, comment_id, vote_type)
-- VALUES (current_user_id, target_comment_id, like_or_dislike_value)
-- ON CONFLICT (user_id, comment_id)
-- DO UPDATE SET vote_type = like_or_dislike_value;
----------------------------------------------------------
-- remove vote query:
----------------------------------------------------------
-- DELETE FROM comment_votes
-- WHERE account_id = current_user_id
-- AND comment_id = target_comment_id;
----------------------------------------------------------
CREATE TABLE IF NOT EXISTS comment_votes (
  account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
  comment_id INT NOT NULL REFERENCES comments(id) ON DELETE CASCADE,
  vote_type INT NOT NULL CHECK (vote_type IN (1, -1)),
  PRIMARY KEY (account_id, comment_id)
);

-- ////////////////////////////////////////////////////////////////
-- open/close time -----------------------------------
----------------------------------------------------------
-- get all days:
-- SELECT day_of_week, open_time, close_time FROM restaurant_hours
-- WHERE restaurant_id = $1
-- ORDER BY day_of_week; 
----------------------------------------------------------
-- post a day:
----------------------------------------------------------
-- INSERT INTO restaurant_hours (restaurant_id, day_of_week, open_time, close_time)
-- VALUES ($1, $2, $3, $4);
-- WHERE role = 'admin'
-- OR role = 'restaurant_owner'
----------------------------------------------------------
-- put a day:
----------------------------------------------------------
-- UPDATE restaurant_hours
-- SET open_time = $1, close_time = $2
-- WHERE restaurant_id = $3
-- AND day_of_week = $4
-- AND role = 'admin'
-- OR role = 'restaurant_owner'
----------------------------------------------------------
-- delete a day (Absent days count as holidays)
----------------------------------------------------------
-- DELETE FROM restaurant_hours 
-- WHERE restaurant_id = $1
-- AND day_of_week = $2
-- AND role = 'admin'
-- OR role = 'restaurant_owner'
-- RETURNING *
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
CREATE table IF NOT EXISTS order (
  id SERIAL PRIMARY KEY,

) 







-- cart -----------------------------------
CREATE table IF NOT EXISTS cart (
  id SERIAL PRIMARY KEY,
  
)














307

