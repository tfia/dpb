# dpb

![](frontend/public/favicon.ico) 

*dpb* provides a set of online pastebin (clipboard) service APIs and corresponding frontend implementations.

The *dpb* backend server is written in Rust with actix-web, and the frontend is written in TypeScript with Next.js.

## Backend

### build

```shell
cargo build --release
```

This will generate the binary file at `target/release/dpb`.

### Configuration

You can pass these arguments to *dpb* backend cli:

```
Options:
  -c, --config <FILE>  Path to the configuration file
  -f, --flush-data     Clear the database before starting
  -h, --help           Print help
  -V, --version        Print version
```

Configuration file is a JSON file with the following fields:

```json
{
    "bind_address": "127.0.0.1",
    "bind_port": 22345,
    "magic": "magic_key"
}
```

- `bind_address`: the address to bind, default is `127.0.0.1`
- `bind_port`: the port to bind, default is `22345`
- `magic`: backend encrypts the timestamp of paste entry and generates key of `/add` response with this magic key, default is `magic_key`

### APIs

- POST `/add`

Request:

```json
{
    "title": "",
    "content": "",
    "expiration": 
}
```

`expiration` is the expiration time of the paste. The unit is second. Leave this field empty or set it to `0` to make the paste never expire.

Response:

```json
{
    "key": ""
}
```

`key` is the unique identifier of the paste, which is used to query the paste content. Frontend can use this key to generate the URL to access the paste.

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

1. `code = 1, reason = ERR_NOT_FOUND, HTTP 404 Not Found`: returns by `/query/{key}` when the key is invalid, which includes decryption failed, key not found in database, paste expired, etc.
2. `code = 2, reason = ERR_INVALID_REQUEST, HTTP 400 Bad Request`: returns by `/add` when the request is invalid, which includes invalid expiration, etc.
3. `code = 3, reason = ERR_INTERNAL_SERVER_ERROR, HTTP 500 Internal Server Error`: returns by any API when an internal error occurs, which includes database error, etc.

## Frontend

### Development

```shell
cd frontend
yarn install
yarn dev
```

This will start a development server at `http://localhost:3000`.

### Production

```shell
cd frontend
yarn install
yarn build
```

or just deploy it on Vercel.

### Configuration

The frontend reads the backend's URL from the environment variable `NEXT_PUBLIC_API_BASE_URL`. You can set this variable in the `.env.local` file.

For example, if you started your backend at `http://127.0.0.1:22345`, you can set the `.env.local` file as:

```
NEXT_PUBLIC_API_BASE_URL=http://127.0.0.1:22345
```
