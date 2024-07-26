# Rust Rocket

## Run the Rocket backend with Docker (Easy)

Run the Rocket backend with its accompanying Postgres database with Docker Compose.
```bash
docker compose up
```

## Run the Rocket backend manually

### Installation

#### Rust

Before you can start the Rocket application, you'll need to install the Rust toolchain.
We recommend https://rustup.rs/

Run rust in dev/nightly version to avoid #feature errors.
```bash
rustup default nightly
```

#### Database

Start up a Postgres database. More information regarding the setup of the database can be found on https://www.postgresql.org/docs/current/tutorial-createdb.html.

#### Diesel

Install the diesel cli by following the tutorial at https://diesel.rs/guides/getting-started.

Use Diesel to create database migrations.

To create migrations either create an empty migration with
```bash
diesel migration generate create_table_name
```
or change the schema.rs file and run
```bash
diesel migration generate --diff-schema
```
to generate a migration from the changes in the schema.rs file.

Create an .env file in rocket root dir to with postgres connection string
```bash
DATABASE_URL=postgres://username:password@localhost/dbname
```
To run the server use
```bash
cargo run
```
