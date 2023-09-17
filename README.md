# URL Shortner

## Technologies

- `Rust`, `actix web`, `tokio` for runtime, `sqlx` to interact with db
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
    "short_url": "tier.app/DaHvis_"
}
```

### Visiting a shortened URL

Once we have our key, which is the unique id after `tier.app/`, we can make a request to `/visit/${key}` url
for it to be validated and redirected to the original url.

**Request** 

```bash
curl --location 'http://localhost:1234/visit/PL9B8g5'
```

If the key is valid and either present in cache or db, the user would be redirect to the original url. If not, it will return 
an error response with a message.

### Health check

`/health-check` endpoint returns a status code of the connected db and cache.

**Request** 

```bash
curl --location 'http://localhost:1234/health-check'
```

**Response**

```json
{
    "cache_is_alive": true,
    "db_is_alive": true,
    "reporting_time": "2023-09-17 05:00:12.967406 UTC"
}
```

> Postman collection has been provided inside [docs/tier.postman_collection.json](docs/tier.postman_collection.json).


## Architecture

The project exposes and wraps everying related to url shortener inside `src/url_shortener` directory (apart from config & configration). This way the code can be exported as a package by just taking the code from `src/url_shortener` and also
can be served as a service that runs on it it's own by simply running [src/main.rs](src/main.rs).  

[src/main.rs](src/main.rs) only enables logging, reads the config file and calls the `UrlShortenerService` and `HttpServer`.  

> `HttpServer` is currently a blank struct, for simplity and scope of this challenge. In production, the `HttpServer`
> can be initiated with it's own configuration (eg: timeouts, tls serving etc).

The [UrlShortenerService](src/url_shortener/service.rs) depends on a database provider and a caching provider. Both have been exposed using traits, therefore it is not bound to `postgres` or `redis` or anything. Any database can implement [DataStore](https://github.com/thearyanahmed/urlshortner/blob/6faaf4ed289673e3b261dfe9a2fecb8ff5080a52/src/url_shortener/service.rs#L24) and any cache can
implement [CacheStore](https://github.com/thearyanahmed/urlshortner/blob/6faaf4ed289673e3b261dfe9a2fecb8ff5080a52/src/url_shortener/service.rs#L33) for the service to work.

By default, a [postgres](src/url_shortener/db/postgres.rs) and [redis](src/url_shortener/cache/redis.rs) driver have been provided.

The service lets the database to handle it's own logic. Eg: which column to query, the service is not concerned about that, and is
handed of the database via traits. So for some reason if a database wishes to call the `original_url` as `actual_url`, it can. Same for the caching.

The service itself exposes some public method, where in those traits are called, along with some logic to provide the desired
service (shortening urls).

## Generating unique keys



## Project structure

write about pre populated keys (system design)
