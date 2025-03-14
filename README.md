# Restaurant API Documentation

## Base URL


http://localhost:4444



## Endpoints

### 1. List All Restaurants
**GET** `/restaurants`

Query Parameters:
- 

limit

 (optional): Number of restaurants to return
- 

offset

 (optional): Number of restaurants to skip

```http
GET /restaurants?limit=10&offset=0
```

Response (200 OK):
```json
[
  {
    "id": 1,
    "name": "Pizza Palace",
    "rating": 4.5,
    "distance": 2.3,
    "tags": ["Italian", "Fast Food"],
    "menu": [
      {
        "name": "Margherita Pizza",
        "price": 10,
        "ingredients": ["Cheese", "Tomato Sauce", "Basil"],
        "is_available": true
      }
    ],
    "image": "https://example.com/pizza_palace.jpg"
  }
]
```

### 2. Create Restaurant
**POST** `/restaurants`

Request Body:
```json
{
  "name": "New Restaurant",
  "rating": 4.5,
  "distance": 2.0,
  "tags": ["Italian", "Pizza"],
  "menu": [
    {
      "name": "Special Pizza",
      "price": 15,
      "ingredients": ["Cheese", "Tomato", "Pepperoni"],
      "is_available": true
    }
  ],
  "image": "https://example.com/image.jpg"
}
```

Response (200 OK):
```text
restaurant added!
```

### 3. Get Single Restaurant
**GET** `/restaurants/{id}`

```http
GET /restaurants/1
```

Response (200 OK):
```json
{
  "id": 1,
  "name": "Pizza Palace",
  "rating": 4.5,
  "distance": 2.3,
  "tags": ["Italian", "Fast Food"],
  "menu": [
    {
      "name": "Margherita Pizza",
      "price": 10,
      "ingredients": ["Cheese", "Tomato Sauce", "Basil"],
      "is_available": true
    }
  ],
  "image": "https://example.com/pizza_palace.jpg"
}
```

### 4. Update Restaurant
**PUT** `/restaurants/{id}`

Request Body:
```json
{
  "id": 1,
  "name": "Updated Pizza Palace",
  "rating": 4.8,
  "distance": 2.3,
  "tags": ["Italian", "Fast Food", "Pizza"],
  "menu": [
    {
      "name": "Super Margherita",
      "price": 12,
      "ingredients": ["Cheese", "Tomato Sauce", "Basil", "Olive Oil"],
      "is_available": true
    }
  ],
  "image": "https://example.com/updated_pizza_palace.jpg"
}
```

Response (200 OK):
Returns the updated restaurant object.

### 5. Delete Restaurant
**DELETE** `/restaurants/{id}`

```http
DELETE /restaurants/1
```

Response (200 OK):
```text
Restaurant 1 deleted
```

## Error Responses

The API may return the following error responses:

- 404 Not Found: Restaurant not found
- 422 Unprocessable Entity: Invalid request data
- 416 Range Not Satisfiable: Database query error
- 403 Forbidden: CORS error

## Example Requests using cURL

1. List restaurants:
```bash
curl -X GET "http://localhost:4444/restaurants?limit=10&offset=0"
```

2. Create restaurant:
```bash
curl -X POST "http://localhost:4444/restaurants" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "New Restaurant",
    "rating": 4.5,
    "distance": 2.0,
    "tags": ["Italian", "Pizza"],
    "menu": [
      {
        "name": "Special Pizza",
        "price": 15,
        "ingredients": ["Cheese", "Tomato", "Pepperoni"],
        "is_available": true
      }
    ],
    "image": "https://example.com/image.jpg"
  }'
```

3. Get single restaurant:
```bash
curl -X GET "http://localhost:4444/restaurants/1"
```

4. Update restaurant:
```bash
curl -X PUT "http://localhost:4444/restaurants/1" \
  -H "Content-Type: application/json" \
  -d '{
    "id": 1,
    "name": "Updated Restaurant",
    "rating": 4.8,
    "distance": 2.0,
    "tags": ["Italian", "Pizza"],
    "menu": [
      {
        "name": "Updated Special Pizza",
        "price": 18,
        "ingredients": ["Cheese", "Tomato", "Pepperoni", "Mushrooms"],
        "is_available": true
      }
    ],
    "image": "https://example.com/updated_image.jpg"
  }'
```

5. Delete restaurant:
```bash
curl -X DELETE "http://localhost:4444/restaurants/1"
```
