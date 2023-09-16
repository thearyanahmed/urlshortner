#!/usr/bin/env bash

# set -x
set -eo pipefail

if ! [ -x "$(command -v redis-cli)" ] ; then
    >&2 echo "redis-cli is not installed. Exiting."
    exit 1
fi

REDIS_PORT="${REDIS_PORT:=6379}"

if [[ $* == *--sd* ]]; then
    >&1 echo "Skipping Docker"
else
    docker run \
        --name my_redis_container \
        -p "${REDIS_PORT}:6379" \
        -d redis

    sleep 1 # Allow some time for the container to start
fi

# Wait for Redis to be available
until redis-cli -h "localhost" -p "${REDIS_PORT}" ping > /dev/null 2>&1; do
    >&2 echo "Redis still unavailable - sleeping"
    sleep 1
done

echo "Redis is up and running on port ${REDIS_PORT}!"

# Export Redis URL for application
export REDIS_URL=redis://localhost:${REDIS_PORT}
