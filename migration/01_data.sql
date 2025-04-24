INSERT INTO restaurant (name, rating, distance, tags, menu, image, city, address)
VALUES
  ('Pasta Paradise', 4.5, 1.2, 
   ARRAY['Italian', 'Pasta', 'Romantic'], 
   ARRAY[
     ROW('Spaghetti Carbonara', 15, ARRAY['pasta', 'eggs', 'bacon', 'parmesan'], true)::foodType,
     ROW('Margherita Pizza', 18, ARRAY['dough', 'tomato sauce', 'mozzarella', 'basil'], true)::foodType,
     ROW('Tiramisu', 8, ARRAY['ladyfingers', 'mascarpone', 'coffee', 'cocoa'], true)::foodType
   ], 
   'pasta_paradise.jpg', 'New York', '123 Main St'),

  ('Burger Barn', 4.2, 0.8, 
   ARRAY['American', 'Burgers', 'Casual'], 
   ARRAY[
     ROW('Classic Cheeseburger', 12, ARRAY['beef patty', 'cheddar', 'lettuce', 'tomato', 'bun'], true)::foodType,
     ROW('Bacon BBQ Burger', 14, ARRAY['beef patty', 'bacon', 'bbq sauce', 'onion rings', 'bun'], true)::foodType,
     ROW('Sweet Potato Fries', 6, ARRAY['sweet potatoes', 'salt', 'oil'], true)::foodType
   ], 
   'burger_barn.jpg', 'Chicago', '456 Oak Ave'),

  ('Sushi Heaven', 4.7, 2.1, 
   ARRAY['Japanese', 'Sushi', 'Fine Dining'], 
   ARRAY[
     ROW('California Roll', 10, ARRAY['crab', 'avocado', 'cucumber', 'rice', 'nori'], true)::foodType,
     ROW('Salmon Nigiri', 12, ARRAY['salmon', 'rice', 'wasabi'], true)::foodType,
     ROW('Miso Soup', 4, ARRAY['tofu', 'seaweed', 'miso paste', 'green onions'], true)::foodType
   ], 
   'sushi_heaven.jpg', 'Los Angeles', '789 Pacific Blvd');

INSERT INTO comments (restaurant_id, name, text, likes, dislikes)
VALUES
  (1, 'Sarah Johnson', 'The carbonara was absolutely divine! Perfectly creamy and flavorful.', 24, 2),
  (1, 'Michael Chen', 'Great atmosphere but the pizza crust was a bit too thick for my taste.', 15, 5),
  (1, 'Emma Williams', 'Best Italian food I''ve had outside of Italy!', 32, 1),
  
  (2, 'David Brown', 'Juicy burgers and crispy fries - exactly what I was craving!', 18, 3),
  (2, 'Jessica Lee', 'The bacon BBQ burger is a must-try. So many delicious flavors!', 27, 0),
  (2, 'Robert Taylor', 'Service was slow but the food made up for it.', 10, 8),
  
  (3, 'Olivia Martinez', 'Fresh sushi and beautiful presentation. Worth every penny!', 42, 0),
  (3, 'James Wilson', 'The salmon melted in my mouth. Excellent quality fish.', 35, 1),
  (3, 'Sophia Anderson', 'Authentic Japanese experience with friendly staff.', 29, 2);