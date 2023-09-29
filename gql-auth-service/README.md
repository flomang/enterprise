### auth-service
An auth GraphQL API that features:
* signup
* signin
* update user
* update user role (only 'admins' can execute)

Users shall have a default role of 'user' in the system unless an admin updates their role. 

## Getting started
* Install [Rust](https://www.rust-lang.org/)
* Install [PostgreSQL](https://www.postgresql.org/) if you don't have it already.
* Install the [Diesel CLI](https://github.com/diesel-rs/diesel/tree/master/diesel_cli) with the `postgres` feature enabled.
* Copy (`cp`) [.env.example](./.env.example) to `.env` within this directory, and change the environment variables accordingly.
* Setup your database by running `diesel database setup`. Make sure it has completed successfully.

## Building
* Build this project with `cargo build`. You are welcome to compile with `--release` if you'd like.

## Running
* Run with `cargo watch -x 'run'`.
* Open a browser window at `localhost:SERVER_PORT` to view the browser GraphiQL IDE. i.e. `SERVER_PORT` value declared in `.env`.

### Crates used  
* [Async-graphql](https://github.com/async-graphql) - async graphQL server framework
* [Actix](https://actix.rs/) - a powerful Actor framework
* [diesel](https://diesel.rs/guides/getting-started.html) - Diesel is an ORM and query builder for retaional databases.

