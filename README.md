# eve-anchor
## Overview

`eve-anchor` is a Discord bot designed to assist with various tasks in the EVE Online game. This README provides instructions on setting up and running the bot locally, as well as running tests.

## Setup

Create a `.secrets` file in the root directory with the following content:

```
ABOT_TOKEN=
APP_ID=
PUBLIC_KEY=
CRATES_TOKEN=
```

## Running the Bot Locally

1. Start a PostgreSQL container:
    ```sh
    docker run --name postgres --detach --env POSTGRES_PASSWORD=precious --publish 5432:5432 postgres:latest
    ```

2. Run the Discord service:
    ```sh
    cd services/discord/
    cargo run
    ```

3. Run the API service:
    ```sh
    cd services/api
    cargo run
    ```

## Running Tests

1. Environment crate tests:
    ```sh
    cd crates/environment
    cargo test -- --test-threads=1
    ```

2. Manager crate tests:
    ```sh
    cd crates/manager
    cargo test -- --test-threads=1
    ```

3. Material LP crate tests:
    ```sh
    cd crates/material_lp
    cargo test -- --test-threads=1
    ```

4. API service tests:
    ```sh
    cd services/api
    cargo test -- --test-threads=1
    ```

5. Discord service tests:
    ```sh
    cd services/discord
    cargo test -- --test-threads=1
    ```