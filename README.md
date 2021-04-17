# Rust TodoMVC

Code for the to do app from the book Rust Web Programming. The code will be gradually improved as I learn Rust more.

## Running the application
- `cargo run`
- `RUST_LOG="info,parser::expression=info,actix_web=info" cargo run` for running it with logging enabled
- `RUST_LOG="info,parser::expression=info,actix_web=info" cargo run --release` for running it in release mode

Note that the code in this repo is missing the `.env` and the `docker-compose.yml` files. You will need to add the connection details for your PostgreSQL database: `DATABASE_URL=postgres://username:password@localhost/to_do
`. The `docker-compose.yml` file is optional, you don't have to run PostgreSQL in a container.

## To do:
- Improve security
- Add more tests
- Refactor to onion architecture
- Replace the frontend code from the book with Rust WebAssembly, using a framework like [Yew](https://yew.rs/docs/en/).

