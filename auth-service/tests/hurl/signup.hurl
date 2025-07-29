POST http://localhost:3000/signup
{}
HTTP 422

POST http://localhost:3000/signup
{
    "email": "garbage@example.com",
    "password": "garbage1234",
    "requires2FA": true
}
HTTP 201

# Second call fails
POST http://localhost:3000/signup
{
    "email": "garbage@example.com",
    "password": "garbage1234",
    "requires2FA": true
}
HTTP 409