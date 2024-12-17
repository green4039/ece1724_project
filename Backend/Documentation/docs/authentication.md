# Authentication

## Signup `POST`
#### API
```
/signup
```
#### Post Body (_Json_)
#### Request:
```Json
{
    "username": "John Wick",
    "email": "test@example.com",
    "password": "123456"
}
```
#### Response: (String with Status Code)
- If email exists:
    - password good: 
        - `STATUS_CODE`: `OK (200)`  
        - `Message`: "`username` Login successful"
    - password bad:
        - `STATUS_CODE`:`BAD_REQUEST (400)` 
        - `Message`: "Invalid password"
- If email does not exist: 
    - `STATUS_CODE`: `CREATED (201)` 
    - `Message`: "User successfully registered"

