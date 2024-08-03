To create a Rocket Diesel documentation for your API, we'll structure it in a way that clearly outlines each endpoint, including the HTTP method, path, purpose, required parameters, and example request/response bodies. This will serve as a comprehensive guide for developers interacting with your API.

### Authentication

Before accessing certain endpoints, users must authenticate themselves. Two endpoints are available for authentication:

#### Login

- **Endpoint**: `POST /auth/login`
- **Purpose**: Authenticate a user and return a token.
- **Request Body**:
```rust
{
    "username": "string",
    "password": "string"
}
```
- **Example Request**:
```http
POST /auth/login HTTP/1.1
Host: example.com
Content-Type: application/json
Authorization: Bearer <token>

{
    "username": "johndoe",
    "password": "password123"
}
```
- **Response**:
```http
HTTP/1.1 200 OK
Content-Type: application/json
Authorization: Bearer <token>

{
    "body": {
        "AuthToken": "<access_token>"
    }
}
```

#### Register

- **Endpoint**: `POST /auth/register`
- **Purpose**: Create a new user account.
- **Request Body**:
```rust
{
    "username": "string",
    "password": "string"
}
```
- **Example Request**:
```http
POST /auth/register HTTP/1.1
Host: example.com
Content-Type: application/json

{
    "username": "janedoe",
    "password": "newpassword"
}
```
- **Response**:
```http
HTTP/1.1 201 Created
Content-Type: application/json

{
    "body": {
        "Message": "User successfully created"
    }
}
```

### Posts

Users can manage posts through several endpoints. Each requires authentication via a valid access token in the `Authorization` header.

#### Get My Post

- **Endpoint**: `GET /post/`
- **Purpose**: Retrieve the authenticated user's latest post.
- **Headers**:
```http
Authorization: Bearer <access_token>
```
- **Response**:
```http
HTTP/1.1 200 OK
Content-Type: application/json

{
    "data": [
        {
            "id": "<post_id>",
            "title": "My First Post",
            "body": "This is my first post.",

        }
    ]
}
```

#### Get My Post

- **Endpoint**: `GET /post/`
- **Purpose**: Retrieve the all post.
- **Headers**:
```http
Authorization: Bearer <access_token>
```
- **Response**:
```http
HTTP/1.1 200 OK
Content-Type: application/json

{
    "data": [
        {
            "id": "<post_id>",
            "title": "My First Post",
            "body": "This is my first post.",

        }
    ]
}
```

#### Create Post

- **Endpoint**: `POST /post/`
- **Purpose**: Create a new post.
- **Headers**:
```http
Authorization: Bearer <access_token>
```
- **Request Body**:
```rust
{
    "title": "New Post Title",
    "body": "This is the content of my new post."
}
```
- **Response**:
```http
HTTP/1.1 201 Created
Content-Type: application/json

{
    "data": {
        "id": "<post_id>",
        "title": "New Post Title",
        "body": "This is the content of my new post.",
        ...
    }
}
```

#### Update Post

- **Endpoint**: `PUT /post/`
- **Purpose**: Update an existing post.
- **Headers**:
```http
Authorization: Bearer <access_token>
```
- **Request Body**:
```rust
{
    "id": "<post_id>",
    "title": "Updated Post Title",
    "body": "This is the updated content of my post."
}
```
- **Response**:
```http
HTTP/1.1 200 OK
Content-Type: application/json

{
    "message": "Post updated successfully",
    "post": {
        "id": "<post_id>",
        "title": "Updated Post Title",
        "body": "This is the updated content of my post.",
        ...
    }
}
```

#### Get All Posts

- **Endpoint**: `GET /post/all`
- **Purpose**: Retrieve all posts.
- **Headers**:
```http
Authorization: Bearer <access_token>
```
- **Response**:
```http
HTTP/1.1 200 OK
Content-Type: application/json

{
    "data": [
        {
            "id": "<post_id_1>",
            "title": "First Post",
            "body": "This is the first post.",
            ...
        },
        {
            "id": "<post_id_2>",
            "title": "Second Post",
            "body": "This is the second post.",
            ...
        }
    ]
}
```

#### Delete Post

- **Endpoint**: `DELETE /post/<post_id>`
- **Purpose**: Delete a post by ID.
- **Headers**:
```http
Authorization: Bearer <access_token>
```
- **Response**:
```http
HTTP/1.1 204 No Content
```

This documentation provides a clear overview of how to interact with your API using Rocket Diesel. It covers authentication, creating, updating, retrieving, and deleting posts. Remember to replace `<access_token>`, `<post_id>`, and other placeholders with actual values when implementing these endpoints.
