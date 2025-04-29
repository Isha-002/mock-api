# Restaurant API Documentation

## Base URL

`http://localhost:4444`

## Connecting to the Database

This API interacts with a database to store and retrieve restaurant information. You'll need to configure your backend application to connect to a database (PostgreSQL).

## Available Endpoints

- GET `/restaurants`
- POST `/restaurants`
- GET `/restaurants/{id}`
- PUT `/restaurants/{id}`
- DELETE `/restaurants/{id}`
- GET `/restaurants/id/comments`
- POST `/restaurants/id/comments`
- PUT `/restaurants/id/comments/comment_id/likes/add`
- PUT `/restaurants/id/comments/comment_id/dislikes/add`
- PUT `/restaurants/id/comments/comment_id/likes/remove`
- PUT `/restaurants/id/comments/comment_id/dislikes/remove`
- GET `/restaurants/city/tag`
- GET `/restaurants/tag/city`

## API Endpoints

### 1. List All Restaurants

**GET** `/restaurants`

Retrieves a list of all restaurants.

**Query Parameters:**

- `limit` (optional): Limits the number of restaurants returned (e.g., `?limit=10`)
- `offset` (optional): Skips a specified number of restaurants (e.g., `?offset=20`)

**Request:**

```http
GET /restaurants?limit=10&offset=20
```

Response of the Get request (200 OK):

```json
[
  {
    "id": "1",
    "name": "Pizza Palace",
    "rating": 4.5,
    "distance": 1.2,
    "tags": ["Italian", "Pizza"],
    "menu": [
      {
        "name": "Margherita",
        "price": 12.99,
        "ingredients": ["Tomato", "Mozzarella", "Basil"],
        "available": true
      }
    ],
    "city": "New York",
    "Address": "something!",
    "image": "http://example.com/pizza-palace.jpg"
  }
]
```

**How to send a get(/restaurants) request in react:**

```javascript
import React, { useState, useEffect } from 'react';

function RestaurantList() {
  const [restaurants, setRestaurants] = useState([]);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const response = await fetch('http://localhost:4444/restaurants');
        if (!response.ok) {
          throw new Error('Failed to fetch data');
        }
        const data = await response.json();
        setRestaurants(data);
      } catch (error) {
        console.error('Error fetching restaurants:', error);
      }
    };

    fetchData();
  }, []);

  return (
    <ul>
      {restaurants.map((restaurant) => (
        <li key={restaurant.id}>{restaurant.name}</li>
      ))}
    </ul>
  );
}

export default RestaurantList;
```

### 2. Create Restaurant

**POST** `/restaurants`

Request Body:

```json
{
  "name": "New Restaurant",
  "rating": 4.5,
  "distance": 2.0,
  "tags": ["Japanese", "Sushi"],
  "menu": [
    {
      "name": "California Roll",
      "price": 8.99,
      "ingredients": ["Rice", "Crab", "Avocado"],
      "available": true
    }
  ],
  "city": "New York",
  "Address": "something!",
  "image": "http://example.com/restaurant.jpg"
}
```

Response (201 Created):

```text
Restaurant added successfully
```

**How to send a post(/restaurants) request in react:**

```javascript
const createRestaurant = async (restaurantData) => {
  try {
    const response = await fetch('http://localhost:4444/restaurants', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(restaurantData),
    });

    if (!response.ok) {
      throw new Error('Failed to create restaurant');
    }

    console.log('Restaurant created successfully!');
  } catch (error) {
    console.error('Error creating restaurant:', error);
  }
};

// Example usage:
const newRestaurant = {
  name: 'My New Restaurant',
  rating: 4.0,
  distance: 3.2,
  tags: ['Burgers', 'American'],
  menu: [],
  city: 'New York',
  Address: 'something!',
  image: 'http://example.com/new-restaurant.jpg'
};

createRestaurant(newRestaurant);
```

### 3. Get Single Restaurant

**GET** `/restaurants/{id}`

Example:

```http
curl -X GET "http://localhost:4444/restaurants/1"
```

Response (200 OK):

```json
{
  "id": "1",
  "name": "Pizza Palace",
  "rating": 4.5,
  "distance": 1.2,
  "tags": ["Italian", "Pizza"],
  "menu": [
    {
      "name": "Margherita",
      "price": 12.99,
      "ingredients": ["Tomato", "Mozzarella", "Basil"],
      "available": true
    }
  ],
  "image": "http://example.com/pizza-palace.jpg"
}
```

### 4. Update Restaurant

**PUT** `/restaurants/{id}`

Request Body:

```json
{
  "name": "Updated Pizza Palace",
  "rating": 4.8,
  "distance": 1.2,
  "cuisines": ["Italian", "Pizza", "Pasta"],
  "menu": [
    {
      "name": "Super Margherita",
      "price": 14.99,
      "ingredients": ["Tomato", "Buffalo Mozzarella", "Fresh Basil"],
      "available": true
    }
  ],
  "image": "http://example.com/updated-pizza-palace.jpg"
}
```

Response (200 OK):
Returns the updated restaurant object.

**How to send a put(/restaurants/{id}) request in react:**

```javascript
const updateRestaurant = async (id, restaurantData) => {
  try {
    const response = await fetch(`http://localhost:4444/restaurants/${id}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(restaurantData),
    });

    if (!response.ok) {
      throw new Error('Failed to update restaurant');
    }

    console.log('Restaurant updated successfully!');
  } catch (error) {
    console.error('Error updating restaurant:', error);
  }
};

// Example usage:
const restaurantId = '123';
const updatedRestaurantData = {
  name: 'Updated Restaurant Name',
  rating: 4.8,
  distance: 1.5,
  cuisines: ['Italian', 'Pizza', 'Pasta'],
  menu: [],
  image: 'http://example.com/updated-restaurant.jpg',
};

updateRestaurant(restaurantId, updatedRestaurantData);
```

### 5. Delete Restaurant

**DELETE** `/restaurants/{id}`

Example:

```http
curl -X DELETE "http://localhost:4444/restaurants/1
```

Response (200 OK):

```text
Restaurant deleted successfully
```

**How to send a delete(/restaurants/{id}) request in react:**

```javascript
const deleteRestaurant = async (id) => {
  try {
    const response = await fetch(`http://localhost:4444/restaurants/${id}`, {
      method: 'DELETE',
    });

    if (!response.ok) {
      throw new Error('Failed to delete restaurant');
    }

    console.log('Restaurant deleted successfully!');
  } catch (error) {
    console.error('Error deleting restaurant:', error);
  }
};

// Example usage:
const restaurantId = '1';
deleteRestaurant(restaurantId);
```

## Error Responses

The API may return the following error responses:

- `404 Not Found`: Restaurant doesn't exist
- `400 Bad Request`: Invalid request body
- `500 Internal Server Error`: Server error
