# dpb

*dpb* provides a set of online paste bin (clipboard) service APIs and corresponding front-end implementations.

## APIs

- POST `/add`

Request:

```json
{
    "title": "",
    "content": "",
    "expiration": 
}
```

Response:

```json
{
    "key": ""
}
```

- GET `/query/{key}`

Response:

```json
{
    "title": "",
    "content": "",
    "created_at": ,
    "expire_at": 
}
```

- Errors

```json
{
    "code": 1,
    "reason": "",
    "message": ""
}
```

Possible errors:

1. `code = 1, reason = ERR_NOT_FOUND, HTTP 404 Not Found`: returns by `/query/{key}` when the key is invalid, which includes decrypt failed, not found in database, paste expired, etc.
2. `code = 2, reason = ERR_INVALID_REQUEST, HTTP 400 Bad Request`: returns by `/add` when the request is invalid, which includes missing required fields, invalid expiration, etc.
3. `code = 3, reason = ERR_INTERNAL_SERVER_ERROR, HTTP 500 Internal Server Error`: returns by any API when an internal error occurs, which includes database error, etc.