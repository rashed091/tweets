# Twitter clone
Let's create a small part of the Twitter API to be able to post, read, and like tweets. The goal is to be able to use our Twitter clone with a massive number of simultaneous fake users. Before you begin, this page assumes the following:

> You have installed Cargo (Rust package manager)

## API design
Our REST API needs to have three endpoints :

> /tweets
- GET: list last 50 tweets
- POST: create a new tweet
> /tweets/:id
- GET: find a tweet by its ID
- DELETE: delete a tweet by its ID
>  /tweets/:id/likes
- GET: list all likes attached to a tweet
- POST: add +1 like to a tweet
- DELETE: add -1 like to a tweet

## Framework
Salvo is a Rust web framework for building scalable and blazingly-fast web applications. It is built on top of the Tokio framework.

## Initial Setup

First things first, let’s create a Rust project using Cargo. To do this, navigate to your desktop or a preferred location and run

```bash
cargo new project_name
```

With the Rust project initialized, let’s proceed to install the necessary dependencies by executing the following commands.

> Add full library using 
```bash
cargo add library_name
```

> Add a specific features of a library
```bash
cargo add  library_name --features feature_name
```
### Here is list of rust creates our projects depends on
```bash
cargo add anyhow
cargo add salvo
cargo add salvo --features cors
cargo add salvo --features anyhow
cargo add salvo --features logging
cargo add tokio --features macros
cargo add tracing
cargo add tracing-subscriber
cargo add serde_json
cargo add serde --features derive
cargo add chrono --features serde
cargo add env_logger
cargo add dotenv
cargo add uuid --features "serde v4"
```



## Getting Started with Diesel
First, let’s add Diesel to our dependencies. We’re also going to use a tool called `.env` to manage our environment variables for us. We’ll add it to our dependencies as well.

```bash
cargo add diesel --features chrono
cargo add diesel --features postgres
cargo add diesel --features r2d2
cargo add diesel --features uuid
```

Diesel provides a separate CLI tool to help manage your project. Since it’s a standalone binary, and doesn’t affect your project’s code directly, we don’t add it to Cargo.toml. Instead, we just install it on our system.

```bash
cargo install diesel_cli --no-default-features --features postgres
```

By default diesel CLI depends on the following client libraries:

- `libpq` for the PostgreSQL backend
- `libmysqlclient` for the Mysql backend
- `libsqlite3` for the SQlite backend

We need to tell Diesel where to find our database. We do this by setting the DATABASE_URL environment variable. On our development machines, we’ll likely have multiple projects going, and we don’t want to pollute our environment. We can put the url in a .env file instead.

```bash
echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
```

Now we are ready to define our database's schema, we need to set up migrations. Migrations are just small SQL scripts that are run in order to set up or tear down the database and alter its schema later down the line. Diesel handles a lot of the process of setting up and running these.

The first thing to do is create the file `diesel.toml` in the project root. This holds config used by the diesel application. Create it with this content:

```
# For documentation on how to configure this file,
# see diesel.rs/guides/configuring-diesel-cli

[print_schema]
file = "src/schema.rs"
```


Now Diesel CLI can set everything up for us. To get started on creating the migrations themselves, just run the following from your project's root directory:



```bash
diesel setup
```

This will create our database (if it didn’t already exist), and create an empty migrations directory that we can use to manage our schema (more on that later).

Now we’re going to write a small CLI that lets us manage a blog (ignoring the fact that we can only access the database from this CLI…). The first thing we’re going to need is a table to store our posts. Let’s create a migration for that:

```bash
diesel migration generate initialize
```

Diesel CLI will create two empty files for us in the required structure. You’ll see output that looks something like this:

```
Creating migrations/20160815133237_create_posts/up.sql
Creating migrations/20160815133237_create_posts/down.sql
```

Migrations allow us to evolve the database schema over time. Each migration can be applied (up.sql) or reverted (down.sql). Applying and immediately reverting a migration should leave your database schema unchanged.

Next, we’ll write the SQL for migrations:

```sql
# up.sql
CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT FALSE
)
```

```sql
# down.sql
DROP TABLE posts
```

We can apply our new migration:

```bash
diesel migration run
```

It’s a good idea to make sure that down.sql is correct. You can quickly confirm that your down.sql rolls back your migration correctly by redoing the migration:

```bash
diesel migration redo
```

Typically the schema module isn’t created by hand, it gets generated by diesel CLI. When we ran diesel setup, a file called diesel.toml was created which tells Diesel to maintain a file at `src/schema.rs` for us. It should look like this:

```rust
// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
```

# Cargo watch
Cargo Watch watches over your project's source for changes, and runs Cargo commands when they occur.

If you've used nodemon, guard, or entr, it will probably feel familiar.

```bash
cargo install cargo-watch
```

By utilizing the cargo-watch binary we installed earlier, you can run the command shown below to initiate the building process for the project, and start the Salvo HTTP server. This will also monitor the source code for any changes, and automatically rebuild the project if any of the relevant files are modified.

```bash
cargo watch -q -c -w src/ -x run
```

## Models
Models are just Rust structs that map to the tables in the database. Diesel uses them to allow abstracted access to the underlying data, automatically converting to/from MySQL data types and Rust data types and handling converting queries to SQL.

We only have one model for our application which is based off of the pageviews table. Here's what it looks like:

## Routes
Routes define the API endpoints for the application. Rocket provides some powerful macros that make defining routes very simple. Here, we'll set up some routes for recording new page views and listing all recorded page views.

Fill `src/routes.rs` with this content:


## CORS
One thing that we need to do if we want this API to be consumable from a web browser is to add CORS headers to our responses. CORS headers are used by web browsers to enforce security on fetched web content. They prevent, for example, a malicious website from making a request to your banking website and reading your private information without your permission.

APIs that are meant to be readable from different domains than the one the user is currently on (cross-domain) must add headers to their responses to indicate what resources are allowed to be fetched and under what conditions.

We're going to set up a very simple CORS implementation that just allows everyone to access everything from anywhere. You can tweak this to have a more fine-tuned setup by including any of the supported headers in responses instead of or in addition to the one we add here.

Our CORS implementation is going to make use of Rocket's Fairings, which are like light-weight middlewares that can alter requests and responses in between them getting received/sent back.

## How to use this template

To use this template as your project starting point, click "Use this template" at the top of this page, or click [here](https://github.com/rashed091/rustful-service/generate).

## Developing Easily
Install dependencies, setup `rustup` env, then start a dev postgres with credentials:

```sh
sudo pacman -S postgresql-libs # or distro equivalent
make setup
source env.sh
make db # runs postgres in a container
```

then run and test in parallel with:

```sh
make run
make test
```

This workflow is the fastest as it does not require waiting for docker builds, nor deal with a local installations of postgres. You just need to install `docker`, `postgresql-libs` to manage migrations in a container, and run against it with what's essentially `cargo run`.

## Using docker-compose
You can develop and test production equivalents without rust, without local postgres, without postgres libs, and without diesel-cli locally.

This is the production equivalent flow:

```sh
# Build the app with clux/muslrust
make compile
# Put the built binary into a container and compose with a db.
# Then, once the db is up, use clux/diesel-cli to run migrations:
source env.sh
make compose
# Verify
make test
```

## Caveats
**NB:** With `docker-compose` our migration would have to wait for postgres to initialize, either via a sleep or a `psql` "select 1" attempt. See `make compose` for more info.

**NB:** The compile step is required before any build step, so `docker-compose up` would fail without it. It's possible to fix this by using a multistep docker build for the app, but it makes local build caching harder.


**NB:** Sometime run command can give you address/port in use error compile. Please follow the steps below to resolve it:

```bash
#Find:
netstat -vanp tcp | grep 5001
lsof -i tcp:5001
lsof -i :5001

#Kill:
kill -9 <PID>
```
