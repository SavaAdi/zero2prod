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
RUST_LOG="sqlx=error,info" TEST_LOG=true cargo test health_check_works | bunyan
```
## How to add migrations

Using `sqlx`:
```bash
sqlx migrate add add_status_to_subscriptions
```
Add sql code into the newly generated .sql files. Then either use:

```bash
sqlx migrate run
```

or:

```bash
SKIP_DOCKER=true ./scripts/init_db.sh
```
Finally, since we are using sqlx we MUST also invoke:

```bash
cargo sqlx prepare -- --bin zero2prod
```
This will let our CI/CD pipeline do an offline check of our migrations
## Known issues when running the test suite:

If you are running `cargo test` on Linux and see errors like

```
thread 'actix-rt:worker' panicked at
'Can not create Runtime: Os { code: 24, kind: Other, message: "Too many open files" }',
```

This is due to a limit enforced by the operating system on the maximum number of open file descriptors
(including sockets) for each process - given that we are now running all tests as part of a single binary,
we might be exceeding it. The limit is usually set to 1024, but you can raise it with ulimit -n X
(e.g. ulimit -n 10000) to resolve the issue.