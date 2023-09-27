The GraphQL backend for the project.

This project was heavily inspired by the [RealWorld](ihttps://realworld-docs.netlify.app/docs/intro) API spec. The original rest API has been ported over to a GraphQL API. An example previous realworld REST request might have looked like this:

```
POST /api/users/login
body:
{
  "user":{
    "email": "jake@jake.jake",
    "password": "jakejake"
  }
}
```

The GraphQL equivalent has been implemented as a mutation: LoginUser. Refer to the GraphQL schema defined by the `BIND_ADDRESS` in the .env file.

## Getting started

* Install [Rust](https://www.rust-lang.org/)
* Install [PostgreSQL](https://www.postgresql.org/) if you don't have it already.
* Install the [Diesel CLI](https://github.com/diesel-rs/diesel/tree/master/diesel_cli) with the `postgres` feature enabled.
* Clone this repo to a folder on your computer.
* Copy (`cp`) [.env.example](./.env.example) to `.env` within this directory, and change the environment variables accordingly to your system.
* Setup your database by running `diesel database setup`. Make sure it has completed successfully.

## Building
* Build this project with `cargo build`. You are welcome to compile with `--release` if you'd like.

## Running
* Run with `cargo watch -x 'run'`.
* Open a browser window at the configured server bind address to view the browser GraphiQL IDE. i.e. `BIND_ADDRESS` value declared in `.env`.

## Database
The postgres database migration files are managed by diesel and are located under the `migrations` folder.

* Create a diesel migration: `diesel migration generate something`
* Running the migration: `diesel migration run`
* Reverting the migration: `diesel migration revert`

Diesel configuration can be found in the diesel.toml file.

## Crates used 
You can view a full list of crates being used in [Cargo.toml](./Cargo.toml), but here are some of the main ones of note:

* [Async-graphql](https://github.com/async-graphql) - async graphQL server framework
* [Actix](https://actix.rs/) - a powerful Actor framework
* [Chrono](https://github.com/chronotope/chrono) - a Date and Time library for Rust
* [Failure](https://rust-lang-nursery.github.io/failure/) - a system for creating and managing errors in Rust
* [Futures](https://docs.rs/futures/0.1.25/futures/) - Zero-cost Futures in Rust
* [jsonwebtoken](https://github.com/Keats/jsonwebtoken) - Create and parses JWT (JSON Web Tokens)
* [libreauth](https://github.com/breard-r/libreauth) - a collection of tools for user authentication
* [Serde](https://serde.rs/) - a framework for serializing and deserializing Rust data structures efficiently and generically
* [Uuid](https://github.com/uuid-rs/uuid) - Generate and parse UUIDs
* [validator](https://github.com/Keats/validator) - Simple validation for Rust structs
* [diesel](https://diesel.rs/guides/getting-started.html) - Diesel is an ORM and query builder for retaional databases.