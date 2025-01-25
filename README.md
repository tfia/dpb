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

- GET `/s/{key}`

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