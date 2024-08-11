# Shorturl

Simple URL shortener service.

For more information on how to use, go to the `/docs` endpoint to access the SwaggerUI inteface.
(http://localhost:7777/docs)

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
