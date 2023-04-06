#!/usr/bin/env bash

set -x           # debug option
set -eo pipefail # suddenly exit if error happened

if ! [ -x "$(command -v psql)" ]; then
	echo >&2 "psql is not installed. Aborting."
	exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
	echo >&2 "sqlx is not installed. Aborting."
	exit 1
fi

DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"
DB_HOST="${POSTGRES_HOST:=localhost}"

docker run \
	-e POSTGRES_USER=${DB_USER} \
	-e POSTGRES_PASSWORD=${DB_PASSWORD} \
	-e POSTGRES_DB=${DB_NAME} \
	-p "${DB_PORT}":5432 \
	-d postgres -N 1000 # increase max number of connections for testing purposes

export PGPASSWORD="${DB_PASSWORD}"

# wait for postgres to be ready
until psql -h ${DB_HOST} -p ${DB_PORT} -U ${DB_USER} -c '\q'; do
	>&2 echo "Postgres is unavailable - sleeping"
	sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT} - running migrations now!"

DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"
export DATABASE_URL
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"
