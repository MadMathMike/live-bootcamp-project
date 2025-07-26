GET http://localhost:3000/
HTTP 200

POST http://localhost:3000/signup
{}
HTTP 422

POST http://localhost:3000/signup
{
    "email": "garbage",
    "password": "garbage",
    "requires2FA": true
}
HTTP 201
