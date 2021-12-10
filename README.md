# rust-echo-server

![Build workflow](https://github.com/docker-sstc/docker-echo-server/actions/workflows/main/badge.svg)
[![Docker pulls](https://img.shields.io/docker/pulls/sstc/echo-server.svg?colorB=green&style=flat)](https://cloud.docker.com/repository/docker/sstc/echo-server)

## Usage

```bash
docker run --rm --init -p 3000:3000 -e RUST_LOG=debug sstc/echo-server
```

## Api

### System api

> All system api are prefix with `/_/`

```bash
curl localhost:3000/_/version # Server version

curl localhost:3000/_/foo # Unknown api would response 404
```

### Echo api

- It would echo request body if request without prefix of system api (works with all http verbs, except OPTIONS and HEAD).

  ```bash
  curl -v localhost:3000/foo \
    -d '{"a":123}'
  ```

  ```console
  ...
  * Connection #0 to host localhost left intact
  {"a":123}
  ```

- It would response the requested status by the header `x-echo-status`.

  ```bash
  curl -v localhost:3000/foo -H "x-echo-status: 400"
  ```

  ```console
  * Connected to localhost (127.0.0.1) port 3000 (#0)
  > POST /foo HTTP/1.1
  > Host: localhost:3000
  > User-Agent: curl/7.64.0
  > Accept: */*
  > x-echo-status: 400
  > Content-Length: 9
  > Content-Type: application/x-www-form-urlencoded
  >
  * upload completely sent off: 9 out of 9 bytes
  < HTTP/1.1 400 Bad Request
  < content-length: 9
  < date: Sat, 16 Feb 2019 08:47:59 GMT
  <
  * Connection #0 to host localhost left intact
  ```

- It would response 400 and `x-echo-status-error` if header `x-echo-status` is invalid.

  ```bash
  curl -v localhost:3000/foo -H "x-echo-status: bar"
  ```

  ```console
  ...
  > x-echo-status: bar
  ...

  ...
  < HTTP/1.1 400 Bad Request
  < x-echo-status-error: invalid digit found in string
  ...
  ```

- It would response `content-type` if request path with extension name.

  ```bash
  curl -v localhost:3000/foo.json -d '{"a":123}'
  ```

  ```console
  ...
  < content-type: application/json
  ...
  ```

## Exceptions

- The OPTIONS (preflight) handle CORS (won't echo).

  ```bash
  curl -v localhost:3000/foo \
    -XOPTIONS \
    -H "Origin: http://foo.bar" \
    -H "Access-Control-Request-Method: FOOMETHOD" \
    -H "Access-Control-Request-Headers: BAR"
  ```

  ```console
  * Connected to localhost (127.0.0.1) port 3000 (#0)
  > OPTIONS /echo.json HTTP/1.1
  > Host: localhost:3000
  > User-Agent: curl/7.64.0
  > Accept: */*
  > Origin: http://foo.bar
  > Access-Control-Request-Method: FOOMETHOD
  > Access-Control-Request-Headers: BAR
  >
  < HTTP/1.1 200 OK
  < content-type: application/json
  < access-control-allow-origin: http://foo.bar
  < access-control-allow-methods: FOOMETHOD
  < access-control-allow-headers: BAR
  < content-length: 0
  < date: Sat, 16 Feb 2019 08:12:50 GMT
  <
  * Connection #0 to host localhost left intact
  ```

- The HEAD (body-less) would response with empty body (won't echo).

  ```bash
  curl -v localhost:3000/foo \
    -XHEAD \
    -d '{"a":123}'
  ```

  ```console
  * Connected to localhost (127.0.0.1) port 3000 (#0)
  > HEAD /foo HTTP/1.1
  > Host: localhost:3000
  > User-Agent: curl/7.64.0
  > Accept: */*
  > Content-Length: 9
  > Content-Type: application/x-www-form-urlencoded
  >
  * upload completely sent off: 9 out of 9 bytes
  < HTTP/1.1 200 OK
  < content-length: 0
  < date: Sat, 16 Feb 2019 08:20:28 GMT
  <
  * Connection #0 to host localhost left intact
  ```

## About

The chinese translation of this app is "複讀機" （meme)

Why? It's human nature.

> 因為這是人類本質。
