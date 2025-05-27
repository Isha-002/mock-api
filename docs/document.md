# Authentication: How to Use

This API uses **token-based authentication** for all protected endpoints.

## 1. Register or Login
- **Register** (`POST /registration`) to create a new user and receive a token.
- **Login** (`POST /login`) with your credentials to receive a token.

**Example:**
```http
POST /login
Content-Type: application/json

{
  "email": "john@example.com",
  "password": "password123"
}
```
**Response:**
```json
"<token>"
```

## 2. Use the Token
For all protected endpoints, you **must** include the token in the `Authorization` header:

```
Authorization: Bearer <token>
```

**Example using curl:**
```sh
curl -H "Authorization: Bearer <token>" http://localhost:4444/restaurants
```

**Example using HTTP:**
```http
GET /restaurants
Authorization: Bearer <token>
```

## 3. Token Details
- The token is a string (Paseto, not JWT) returned as a plain string in the response body.
- The token must be sent exactly as received, with the `Bearer` prefix and a space.
- The token is required for all endpoints that modify data or access user-specific information (e.g., creating, updating, deleting, posting comments, orders, etc.).
- If the token is missing, invalid, or expired, you will receive a `401 Unauthorized` error.

## 4. Example Full Flow
1. **Register:**
    ```http
    POST /registration
    Content-Type: application/json

    {
      "name": "Alice",
      "email": "alice@example.com",
      "password": "secret",
      "phone_number": "1234567890",
      "role": "customer"
    }
    ```
    **Response:**  `"<token>"`

2. **Login (if already registered):**
    ```http
    POST /login
    Content-Type: application/json

    {
      "email": "alice@example.com",
      "password": "secret"
    }
    ```
    **Response:**  `"<token>"`

3. **Use the token in requests:**
    ```http
    GET /restaurants
    Authorization: Bearer <token>
    ```

    ```http
    POST /restaurants
    Authorization: Bearer <token>
    Content-Type: application/json

    {
      "name": "Pizza Place",
      "rating": 4.5,
      "distance": 2.3,
      "tags": ["pizza", "italian"],
      "image": "url",
      "address": "123 Main St",
      "city": "New York",
      "location": [40.7128, -74.0060]
    }
    ```

## 5. Common Errors
- `401 Unauthorized`: Missing, invalid, or expired token.
- `403 Forbidden`: You do not have permission for this action (e.g., not an admin/owner).
- `400 Bad Request`: Malformed or missing data.

---

## Database
- Uses PostgreSQL.


## Command-Line Arguments
- `-d`, `--db-url` — Set the database connection URL.
- `--reset` — Reset (delete) all database tables.
- `--data` — Insert sample (fake) data into the database.

---

## API Endpoints (Summary)

### Authentication
- `POST /registration` — Register a new user.
- `POST /login` — Login and receive a token.

### Restaurants
- `GET /restaurants` — List restaurants (supports pagination). **(Requires: Authorization header with Bearer token for POST/PUT/DELETE)**
- `POST /restaurants` — Create a new restaurant (admin/owner only). **(Requires: Authorization)**
- `GET /restaurants/{id}` — Get a single restaurant.
- `PUT /restaurants/{id}` — Update a restaurant (admin/owner only). **(Requires: Authorization)**
- `DELETE /restaurants/{id}` — Delete a restaurant (admin/owner only). **(Requires: Authorization)**
- `GET /restaurants/city/{city}` — Search restaurants by city.
- `GET /restaurants?tag=...&city=...` — Search restaurants by tag and city.

### Restaurant Hours
- `GET /restaurants/hours/{id}` — Get open hours.
- `POST /restaurants/hours` — Add open hours (admin/owner only). **(Requires: Authorization)**
- `PUT /restaurants/hours` — Update open hours (admin/owner only). **(Requires: Authorization)**
- `DELETE /restaurants/hours` — Delete open hours (admin/owner only). **(Requires: Authorization)**

### Food
- `GET /restaurants/{id}/food` — Get menu for a restaurant.
- `POST /restaurants/food` — Add food (admin/owner only). **(Requires: Authorization)**
- `PUT /restaurants/food` — Update food (admin/owner only). **(Requires: Authorization)**
- `DELETE /restaurants/food` — Delete food (admin/owner only). **(Requires: Authorization)**

### Orders
- `GET /order/{id}` — Get customer orders. **(Requires: Authorization)**
- `POST /order` — Create a cart. **(Requires: Authorization)**
- `PUT /order` — Add item to cart. **(Requires: Authorization)**

### Comments
- `GET /restaurants/{id}/comments` — Get comments for a restaurant. **(Requires: Authorization)**
- `POST /restaurants/{id}/comments` — Add a comment. **(Requires: Authorization)**
- `DELETE /restaurants/comments/{id}/delete` — Delete a comment. **(Requires: Authorization)**
- `POST /restaurants/comments/vote` — Vote on a comment. **(Requires: Authorization)**

### Owners
- `GET /restaurant/owner` — Get owner info. **(Requires: Authorization)**
- `POST /restaurant/owner` — Register owner. **(Requires: Authorization)**
- `PUT /restaurant/owner` — Update owner national ID. **(Requires: Authorization)**
- `PUT /restaurant/owner/replace` — Replace owner. **(Requires: Authorization)**

### File Uploads
- `POST /restaurants/{id}/upload` — Upload restaurant profile image. **(Requires: Authorization)**
- `POST /restaurants/{id}/food/{food_id}/upload` — Upload food image. **(Requires: Authorization)**
- `GET /upload` — Serve uploaded files.


# API Routes

---

## Authentication

### Register
POST /registration
```json
{
  "name": "John Doe",
  "email": "john@example.com",
  "password": "password123",
  "phone_number": "1234567890",
  "role": "customer" // or "restaurant_owner", "admin"
}
```
**Success:**
```json
"<token>"
```
**Errors:**
- 400: Missing parameters, invalid data
- 409: Account already exists

### Login
POST /login
```json
{
  "email": "john@example.com", // or "phone_number"
  "password": "password123"
}
```
**Success:**
```json
"<token>"
```
**Errors:**
- 400: Missing parameters, invalid data
- 401: Wrong credentials

---

## Restaurants

### List Restaurants
GET /restaurants?limit=10&offset=0
**Response:**
```json
[
  {
    "id": 1,
    "name": "Pizza Place",
    "rating": 4.5,
    "distance": 2.3,
    "tags": ["pizza", "italian"],
    "image": "url",
    "address": "123 Main St",
    "city": "New York",
    "location": [40.7128, -74.0060]
  }
]
```
**Errors:**
- 400: Missing/invalid parameters

### Create Restaurant
POST /restaurants
**Headers:**
Authorization: Bearer <token>
```json
{
  "name": "Pizza Place",
  "rating": 4.5,
  "distance": 2.3,
  "tags": ["pizza", "italian"],
  "image": "url",
  "address": "123 Main St",
  "city": "New York",
  "location": [40.7128, -74.0060]
}
```
**Success:**
"restaurant added!"
**Errors:**
- 401: Unauthorized
- 400: Invalid data

### Get Single Restaurant
GET /restaurants/{id}
**Response:**
```json
{
  "id": 1,
  "name": "Pizza Place",
  "rating": 4.5,
  "distance": 2.3,
  "tags": ["pizza", "italian"],
  "image": "url",
  "address": "123 Main St",
  "city": "New York",
  "location": [40.7128, -74.0060]
}
```
**Errors:**
- 404: Not found

### Update Restaurant
PUT /restaurants/{id}
**Headers:**
Authorization: Bearer <token>
```json
{
  "id": 1,
  "name": "Pizza Place",
  "rating": 4.7,
  "distance": 2.1,
  "tags": ["pizza", "italian"],
  "image": "url",
  "address": "123 Main St",
  "city": "New York",
  "location": [40.7128, -74.0060]
}
```
**Success:**
```json
{
  "id": 1,
  "name": "Pizza Place",
  "rating": 4.7,
  "distance": 2.1,
  "tags": ["pizza", "italian"],
  "image": "url",
  "address": "123 Main St",
  "city": "New York",
  "location": [40.7128, -74.0060]
}
```
**Errors:**
- 401: Unauthorized
- 400: Invalid data

### Delete Restaurant
DELETE /restaurants/{id}
**Headers:**
Authorization: Bearer <token>
**Success:**
"Restaurant {id} deleted"
**Errors:**
- 401: Unauthorized
- 404: Not found

### Search by City
GET /restaurants/city/{city}
**Response:**
Array of restaurants (see above)

### Search by Tag
GET /restaurants?tag=pizza&city=New York
**Response:**
Array of restaurants (see above)

---

## Restaurant Hours

### Get Hours
GET /restaurants/hours/{id}
**Response:**
```json
[
  {
    "restaurant_id": 1,
    "day_of_week": "Monday",
    "open_time": "09:00:00",
    "close_time": "22:00:00"
  }
]
```

### Add Hours
POST /restaurants/hours
**Headers:**
Authorization: Bearer <token>
```json
{
  "restaurant_id": 1,
  "day_of_week": "Monday",
  "open_time": "09:00:00",
  "close_time": "22:00:00"
}
```
**Success:**
"hours added!"

### Update Hours
PUT /restaurants/hours
**Headers:**
Authorization: Bearer <token>
```json
{
  "restaurant_id": 1,
  "day_of_week": "Monday",
  "open_time": "10:00:00",
  "close_time": "23:00:00"
}
```
**Success:**
```json
{
  "restaurant_id": 1,
  "day_of_week": "Monday",
  "open_time": "10:00:00",
  "close_time": "23:00:00"
}
```

### Delete Hours
DELETE /restaurants/hours
**Headers:**
Authorization: Bearer <token>
```json
{
  "restaurant_id": 1,
  "day_of_week": "Monday",
  "open_time": "10:00:00",
  "close_time": "23:00:00"
}
```
**Success:**
"food deleted"

---

## Food

### Get Menu
GET /restaurants/{id}/food
**Response:**
```json
[
  {
    "id": 1,
    "restaurant_id": 1,
    "name": "Margherita Pizza",
    "image": "url",
    "tag": "pizza",
    "price": 12,
    "discount": false,
    "discount_price": null,
    "ingredient": ["cheese", "tomato"],
    "available": true
  }
]
```

### Add Food
POST /restaurants/food
**Headers:**
Authorization: Bearer <token>
```json
{
  "restaurant_id": 1,
  "name": "Margherita Pizza",
  "image": "url",
  "tag": "pizza",
  "price": 12,
  "discount": false,
  "discount_price": null,
  "ingredient": ["cheese", "tomato"],
  "available": true
}
```
**Success:**
"food added!"

### Update Food
PUT /restaurants/food
**Headers:**
Authorization: Bearer <token>
```json
{
  "id": 1,
  "restaurant_id": 1,
  "name": "Margherita Pizza",
  "image": "url",
  "tag": "pizza",
  "price": 10,
  "discount": true,
  "discount_price": 8,
  "ingredient": ["cheese", "tomato"],
  "available": true
}
```
**Success:**
Food object (see above)

### Delete Food
DELETE /restaurants/food
**Headers:**
Authorization: Bearer <token>
```json
{
  "id": 1,
  "restaurant_id": 1,
  "name": "Margherita Pizza",
  "image": "url",
  "tag": "pizza",
  "price": 10,
  "discount": true,
  "discount_price": 8,
  "ingredient": ["cheese", "tomato"],
  "available": true
}
```
**Success:**
"food deleted"

---

## Orders

### Get Customer Orders
GET /order/{id}
**Headers:**
Authorization: Bearer <token>
**Response:**
```json
{
  "items": [
    {
      "id": 1,
      "order_id": 1,
      "restaurant_id": 1,
      "food_id": 1,
      "quantity": 2,
      "account_id": "uuid",
      "name": "Margherita Pizza",
      "image": "url",
      "price": 12,
      "discount_price": null
    }
  ],
  "account_id": "uuid",
  "order_id": 1,
  "status": "Cart",
  "total_price": 24,
  "total_discounted_price": 24
}
```

### Create Cart
POST /order
**Headers:**
Authorization: Bearer <token>
**Success:**
"cart created!"

### Add to Cart
PUT /order
**Headers:**
Authorization: Bearer <token>
```json
{
  "order_id": 1,
  "restaurant_id": 1,
  "food_id": 1,
  "quantity": 2,
  "name": "Margherita Pizza",
  "image": "url",
  "price": 12,
  "discount_price": null
}
```
**Success:**
"cart updated!"

---

## Comments

### Get Comments
GET /restaurants/{id}/comments
**Headers:**
Authorization: Bearer <token>
**Response:**
```json
[
  {
    "id": 1,
    "restaurant_id": 1,
    "account_id": "uuid",
    "text": "Great food!",
    "rating": 5,
    "created_on": "2024-05-01T12:00:00",
    "name": "John Doe",
    "likes": 10,
    "dislikes": 0,
    "current_user_vote": 1
  }
]
```

### Add Comment
POST /restaurants/{id}/comments
**Headers:**
Authorization: Bearer <token>
```json
{
  "restaurant_id": 1,
  "text": "Great food!",
  "rating": 5,
  "created_on": "2024-05-01T12:00:00"
}
```
**Success:**
"comment added!"

### Delete Comment
DELETE /restaurants/comments/{id}/delete
**Headers:**
Authorization: Bearer <token>
**Success:**
"Deleted"

### Vote on Comment
POST /restaurants/comments/vote?id=1&vote=1
**Headers:**
Authorization: Bearer <token>
**Success:**
"liked!"

---

## Owners

### Get Owner
GET /restaurant/owner
**Headers:**
Authorization: Bearer <token>
**Response:**
```json
{
  "name": "John Doe",
  "phone_number": "1234567890",
  "email": "john@example.com",
  "national_id": "A1234567"
}
```

### Register Owner
POST /restaurant/owner
**Headers:**
Authorization: Bearer <token>
```json
{
  "restaurant_id": 1,
  "national_id": "A1234567"
}
```
**Success:**
"owner registered!"

### Update Owner National ID
PUT /restaurant/owner
**Headers:**
Authorization: Bearer <token>
```json
{
  "restaurant_id": 1,
  "account_id": "uuid",
  "national_id": "A1234567"
}
```
**Success:**
"owner national id updated!"

### Replace Owner
PUT /restaurant/owner/replace
**Headers:**
Authorization: Bearer <token>
```json
{
  "restaurant_id": 1,
  "account_id": "uuid",
  "national_id": "A1234567"
}
```
**Success:**
"owner replaced!"

---

## File Uploads

### Upload Restaurant Profile Image
POST /restaurants/{id}/upload
**Headers:**
Authorization: Bearer <token>
Content-Type: multipart/form-data
**Body:**
- file: image file
**Success:**
"image added"

### Upload Food Image
POST /restaurants/{id}/food/{food_id}/upload
**Headers:**
Authorization: Bearer <token>
Content-Type: multipart/form-data
**Body:**
- file: image file
**Success:**
"image added"

### Serve Uploaded Files
GET /upload/{filename}
**Response:**
- Returns the file


