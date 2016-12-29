# Rust Blog API

I have been interested in learning [Rust](https://www.rust-lang.org/en-US/) so I
decided to build a simple REST API for a blog using [Rocket](https://rocket.rs/)
and [Diesel](https://diesel.rs/). This is still a work in progress but overall
it has been a fun project!

## Set Up

*In order to run this application, make sure you have Postgres installed and
running.*

If you do not already have Rust nightly installed execute the following command.

```bash
curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly
```

Once you have Rust nightly installed, install `diesel_cli`.

```bash
cargo install diesel_cli
```

Now that you have `diesel_cli` installed, clone this repository and add a `.env`
file containing a `DATABASE_URL` environment variable.

```bash
git clone https://github.com/zacharygolba/rust-blog-api
cd rust-blog-api
echo DATABASE_URL=postgres://postgres@localhost/rust_blog_api > .env
```

Now create the database specified in the `DATABASE_URL` environment variable and
run any pending migrations with the following command.

```bash
diesel database setup
```

## Running

After your environment is setup, you can run the application with the following
command.

```bash
cargo run
```

## Deploying

To deploy the application, first create an executable binary.

```bash
cargo build --release
```

After the build is complete, we can find our application in the
`./target/release` directory.
