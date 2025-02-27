#!/usr/bin/env bash
# -*- mode: bash-ts; coding: utf-8; fill-column: 80; -*-

set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  exit 1
fi

DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"

if [[ -z "${SKIP_DOCKER}" ]]; then
    RUNNING_POSTGRES_CONTAINER=$(docker ps --filter 'name=postgres' --format '{{.ID}}')
    if [[ -n $RUNNING_POSTGRES_CONTAINER ]]; then
        echo >&2 "there is a postgres container already running, kill it with"
        echo >&2 "    docker kill ${RUNNING_POSTGRES_CONTAINER}"
        exit 1
    fi

    docker run \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${DB_PASSWORD} \
        -e POSTGRES_DB=${DB_NAME} \
        -p "${DB_PORT}":5432 \
        -d \
        --name "postgres_$(date '+%s')" \
        postgres \
        -N 1000
fi

until PGPASSWORD="${DB_PASSWORD}" psql -h "localhost" -U\
    "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q';
do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1
done

echo "Postgres is up and running on port ${DB_PORT}!"

export \
    "DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/"\
"${DB_NAME}"

sqlx database create
sqlx migrate run

echo "Postgres has been migrated, ready to go!"
