# URL Shortner

## Technologies

- `Rust`, `actix web`, tokio for runtime
- `Docker` for running `postgres` and `redis` containers
- `Postgres` for primary db
- `Redis` for caching

## Running the project

The project comes with some `make *` commands. To get started, we should first boot up the database and redis. 
This can be done using the following commands.

```bash
make db-init
make redis-init
```

Running (development mode)

```bash
make run

# we can also watch for changes in the codebase by running
# make watch
```

## API

The service exposes 1 `health check` endpoint and 2 other endpoints `/shorten` and `/visit`.  

### Shortening an URL

The endpoint requires `url` as data, which needs to be a valid url. it is a post method. Upon given a valid url, it will return
a unique key with the base url (configured). The base url has been configured for `tier.app`.

**Request** 

```bash
curl --location 'http://localhost:1234/shorten' \
--header 'Content-Type: application/json' \
--data '{
    "url" : "https://apple.com"
}'

```
**Response**

```json
{
    "url": "tier.app/DaHvis_"
}
```




write about pre populated keys (system design)