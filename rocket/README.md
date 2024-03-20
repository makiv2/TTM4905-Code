Run rust in dev/nightly version to avoid #feature errors.

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
