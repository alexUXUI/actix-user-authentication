# actix-user-service

> 🚧 work in progress

Minimal user management service with functionality to create a user, log a user in, fetch all users, or fetch a user by id. 

---

### API

### `/app`
#### `/login` | `POST` -> User with JWT
Request
```shell
curl -X POST \
-H "Content-type: application/json" \
-d '{"name": "paulo", "password": "123" }' \ 
http://localhost:3000/users/login
```
2XX Response 
```json
{
    "user_logged_in": {
        "name":"paulo",
        "email":"paulo@email.com",
        "jwt":"<JWT>"
    }
}
```

4XX Response 

```json
{
    "message": "Could not log user in",
    "error": "User does not exist" | "Incorrect password"
}
```


### `/users`
#### `/all` | `GET` ->  All users 
Request
```shell
curl -X GET \
-H "Authorization: <JWT>" \
http://localhost:3000/users/all 
```
2XX Response 
```json
{
    "users": [
        {
            "id": 2,
            "name": "Alex",
            "email": "alex@email.com"
        }
    ],
}
```

#### `/create` | `POST` -> Creates a new user
Request
```shell
curl -X POST \
-H "Content-type: application/json" \
-H "Authorization: <JWT>" \
-d '{"name": "clara", "password": "123", "email": "clara@email.com" }' \
http://localhost:3000/users/create
```
2XX Response
```json
{
    "new_user": {
        "name": "clara",
        "email": "clara@email.com"
    }
}
```

4XX Response
```json
{
    "message": "Failed to create user",
    "error": "Email already in use"
}
``` 
#### `/{id}` | `GET` -> Gets a user by ID
Request
```shell
curl -X GET \
-H "Content-type: application/json" \
-H "Authorization: <JWT>" \
http://localhost:3000/users/2
```
Response 
```json
{
    "user": {
        "id": 2,
        "name":"Alex",
        "email":"alexbennettuxui@gmail.com",
    }
}
```

### Tasks:
1) Tests

2) Use a third-party authentication strategy

3) Make routes for:
- log out
- reauth
- delete
- update
- reset pw

4) Cookies and cookie-based sessions

5) Better error-handling scheme with better types and middleware


