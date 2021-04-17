# Rust TodoMVC

Code for the to do app from the book Rust Web Programming. The code will be gradually improved as I learn Rust more.

## Running the application
- `cargo run`
- `RUST_LOG="info,parser::expression=info,actix_web=info" cargo run` for running it with logging enabled
- `RUST_LOG="info,parser::expression=info,actix_web=info" cargo run --release` for running it in release mode

Note that the code in this repo is missing the `.env` and the `docker-compose.yml` files. You will need to add the connection details for your PostgreSQL database: `DATABASE_URL=postgres://username:password@localhost/to_do
`. The `docker-compose.yml` file is optional, you don't have to run PostgreSQL in a container. If you choose to run it in a container, the file looks something like this:
```
version: "3.7"

services:

  postgres:
    container_name: 'to-do-postgres'
    image: 'postgres:11.2'
    restart: always
    ports:
      - '5432:5432'
    environment:
      - 'POSTGRES_USER=username'
      - 'POSTGRES_DB=to_do'
      - 'POSTGRES_PASSWORD=password'
```

## To do:
- Improve security
- Add more tests
- Refactor to onion architecture
- Replace the frontend code from the book with Rust WebAssembly, using a framework like [Yew](https://yew.rs/docs/en/).

