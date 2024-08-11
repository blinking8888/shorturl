# Shorturl

Simple URL shortener service written in pure Rust with the [`axum`](https://github.com/tokio-rs/axum) framework.

For more information on how to use, go to the [`/docs`](http://localhost:7777/docs) endpoint to access the SwaggerUI inteface.
SwaggerUI documentation was generated using [`utoipa`](https://github.com/juhaku/utoipa) with [`utoipa-swagger-ui`](https://github.com/juhaku/utoipa/tree/master/utoipa-swagger-ui)

## Setup

Run locally by doing a `cargo run`.  This will be hosted locally at http://localhost:7777.

## Shorten the URL

Use the `/shorten` endpoint and pass in the `url` as shown below in `curl`.

```sh
curl -X 'POST' \
  'http://localhost:7777/shorten' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -d '{
  "url": "https://www.example.com"
}'
```
