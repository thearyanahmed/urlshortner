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

The [generate_unique_key](https://github.com/thearyanahmed/urlshortner/blob/b99ec0e6db53d35e595e2a43adc4567642cb363c/src/url_shortener/service.rs#L137-L159) function takes an input string, applies a cryptographic hash, combines it with a random value (salt), encodes it in base64, and then truncates it to a specified length. This process ensures that the generated key is unique for each input and is of a specified length.

> I am trauncating the result of the final value. It can create a colluision theorytically. Though salt has been added. A collusion
> checking mechanism using a (pseudo) **rate limiter** has been implemented.  
>
> This apporach creates multiple quries to the database. A better apporach would be to pregenerate the keys, putting them in hashmap / similar datastructure and taking them from that set. Sort of like a pool of values, when we need a key, we take from
> theat pool and mark it as `used` / similar to indicate that key is no longer valid. 
> The pool should be thread safe in case of multiple client connections trying to take a key at the same time. 

## Known issues

There are some known issues, they were not fixed due to the lack of time. I'm listing them here,

1. More tests can be added, only integration tests are not present.
2. The approach of integration test can be improved. Now, on every test iteration it create a database but does not database. 
3. Benchmarks tests are not present.
4. Resulting in lots of database. This can cause a problem if tests are frequently ran and can hangup file descriptors.
5. No cache manager/database manager was provided, right now in the main file it is hardcoded which driver we are using. But we can use something [similar to this (different project)](https://github.com/thearyanahmed/lucy/blob/29f3c2547837c213a4973844e758c5722fe2364b/src/lib.rs#L33-L38).
6. Wanted to take an approach to registering a key-value to cache in a background tasks but due to the lack of time, didn't get the chance to work with background tasks.
7. Error handling was done pretty simply with Result<T,String> or Result<T, sqlx::Error>, crates liek `anyhow::error` could've helped improve this approach.
8. Right now, the caching is (also) acting as a secondary database, we might not need to have all the URLs in cache, we can approach LRU or LFU caching based on the requirement (to come).
9. Actix handles `SIGTERM` and `SIGKILL`, I did not handle it to do something particular.
10. API versioning was left out.
11. Reading sensitive data from / for environment is not plain text, eg: database passwords, this can be imrpvoed by `secrecy::Secret` crate, that keep logs / tracing to collect sentivite data.
12. SQL constrains were based on a simple assumtion.
13. The `visits` table acts as a log, so we can aggregate in the future if needed. Storing the simplest unit of data.
14. SQL builder can help in the db drivers level.  
