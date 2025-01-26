INSERT INTO restaurant (name, rating, distance, tags, menu, image)
VALUES 
-- Restaurant 1: Pizza Palace
(
    'Pizza Palace', 
    4.5, 
    2.3, 
    ARRAY['Italian', 'Fast Food'], 
    ARRAY[
        ROW('Margherita Pizza', 10, ARRAY['Cheese', 'Tomato Sauce', 'Basil'], true)::food,
        ROW('Pepperoni Pizza', 12, ARRAY['Cheese', 'Tomato Sauce', 'Pepperoni'], true)::food,
        ROW('BBQ Chicken Pizza', 14, ARRAY['BBQ Sauce', 'Chicken', 'Onions', 'Cheese'], true)::food
    ], 
    'https://example.com/pizza_palace.jpg'
),
-- Restaurant 2: Burger Haven
(
    'Burger Haven', 
    4.2, 
    1.8, 
    ARRAY['American', 'Fast Food'], 
    ARRAY[
        ROW('Classic Cheeseburger', 8, ARRAY['Beef Patty', 'Cheese', 'Lettuce', 'Tomato'], true)::food,
        ROW('Veggie Burger', 7, ARRAY['Lettuce', 'Tomato', 'Veggie Patty', 'Cheese'], false)::food,
        ROW('Bacon Burger', 9, ARRAY['Beef Patty', 'Bacon', 'Cheese', 'BBQ Sauce'], true)::food
    ], 
    'https://example.com/burger_haven.jpg'
),
-- Restaurant 3: Sushi World
(
    'Sushi World', 
    4.8, 
    3.1, 
    ARRAY['Japanese', 'Seafood'], 
    ARRAY[
        ROW('California Roll', 15, ARRAY['Crab', 'Avocado', 'Cucumber'], true)::food,
        ROW('Salmon Nigiri', 18, ARRAY['Salmon', 'Rice', 'Seaweed'], true)::food,
        ROW('Tuna Sashimi', 20, ARRAY['Tuna', 'Soy Sauce'], true)::food
    ], 
    'https://example.com/sushi_world.jpg'
),
-- Restaurant 4: Taco Fiesta
(
    'Taco Fiesta', 
    4.6, 
    2.0, 
    ARRAY['Mexican', 'Street Food'], 
    ARRAY[
        ROW('Chicken Taco', 5, ARRAY['Tortilla', 'Chicken', 'Salsa', 'Lettuce'], true)::food,
        ROW('Beef Taco', 6, ARRAY['Tortilla', 'Beef', 'Cheese', 'Tomato'], true)::food,
        ROW('Veggie Taco', 4, ARRAY['Tortilla', 'Beans', 'Lettuce', 'Sour Cream'], true)::food
    ], 
    'https://example.com/taco_fiesta.jpg'
),
-- Restaurant 5: Pasta House
(
    'Pasta House', 
    4.3, 
    3.5, 
    ARRAY['Italian', 'Fine Dining'], 
    ARRAY[
        ROW('Spaghetti Carbonara', 13, ARRAY['Spaghetti', 'Egg', 'Cheese', 'Bacon'], true)::food,
        ROW('Penne Alfredo', 12, ARRAY['Penne Pasta', 'Cream', 'Parmesan', 'Chicken'], true)::food,
        ROW('Lasagna', 15, ARRAY['Pasta Sheets', 'Tomato Sauce', 'Ground Beef', 'Cheese'], true)::food
    ], 
    'https://example.com/pasta_house.jpg'
);
