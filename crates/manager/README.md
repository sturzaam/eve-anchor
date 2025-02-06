

## Install the manager

        cargo install

## Start the database

        docker run --name postgres --detach --env POSTGRES_PASSWORD=precious  --publish 5432:5432  postgres:latest

## Run the tests

        cargo test --test unit --test integration --  --test-threads=1