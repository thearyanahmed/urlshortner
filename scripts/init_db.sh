#!/usr/bin/env bash

# set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ] ; then
	>&2 echo "psql is not install. exiting"
	exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
	# shellcheck disable=SC2028
	>&2 echo "sqlx not installed. to install, run \n"
	>&2 echo "cargo install --version=0.7 sqlx-cli --no-default-features --features postgres"

	exit 1
fi

DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=urlshortner}"
DB_PORT="${POSTGRES_PORT:=54321}"

if [[ $* == *--sd* ]]; then
    >&1 echo "skipping docker"
else
    docker run \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${DB_PASSWORD} \
        -e POSTGRES_DB=${DB_NAME} \
        -p "${DB_PORT}":5432 \
        -d postgres postgres -N 1000

    #	 \ postgres -N 1000 doesn't work

    #postgres -N 1000 is user to increase maximum number of connections for testing purpose
fi

export PGPASSWORD="${DB_PASSWORD}"

# sleep 1, container needs a bit of time to start.
sleep 1

until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
	>&2 echo "postgres still unavailable - sleeping"
	sleep 1
done

echo "postgres online"

DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}

>&1 echo "DB_URL = ${DATABASE_URL}"

>&1 echo "Postgres is up and running on port ${DB_PORT}!"

sqlx database create
sqlx migrate run

>&1 echo "postgres has been migrated."

