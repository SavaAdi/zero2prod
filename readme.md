## Pre-requisite

You'll need to install:

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)

Launch a (migrated) Postgres database via Docker:

```bash
./scripts/init_db.sh
```

Allow to skip Docker if a dockerized Postgres database is already running

```bash
SKIP_DOCKER=true ./scripts/init_db.sh
```

## How to build

Using `cargo`:

```bash
cargo build
```

## How to test

Using `cargo`:

```bash
cargo test 
```

or

```bash
TEST_LOG=true cargo test health_check_works | bunyan
```
## How to add migrations

Using `sqlx`:
```bash
sqlx migrate add add_status_to_subscriptions
```
Add sql code into the newly generated .sql files. Then

```bash
sqlx migrate run
```