# axum-api-server7

axum 0.7+ api server &amp; ethers.rs

## Why Axum

Quote from Discord: Although axum is less mature than actix-web it should have a brighter future because it is part of the tokio project and integrating with other libraries should be easier. It is also based on existing foundations used by other web servers, i.e. tower.
Using actix-web adds complications because it is using its own actix-rt runtime. It is based on tokio but it does its own thing with threads which may cause some incompatibilities with other projects. Libraries like sqlx and sea-orm have feature flags to use this runtime but most other projects typically just support tokio only. You can run actix-web under the tokio runtime but then you lose support for actix actors and websockets. I think it can be made to work but it isn't something that you really want people to deal with if they are just learning.

## Tutorial and Source Code

Rust Axum Full Course - Web Development (GitHub repo updated to Axum 0.7) by Jeremy Chone
https://www.youtube.com/watch?v=XZtlD_m59sM
https://github.com/jeremychone-channel/rust-axum-course

Mastering Rust Web Services: From Axum to CRUD Operations by Ardan Labs(Herbert Wolverson)
https://www.youtube.com/watch?v=JUWSy9pXgMQ
https://github.com/thebracket/webinar_axumcrud

## Tech Stack

Database: SQLite in memory
Data Model: SQLx
REST API: AXUM
Web View: JavaScript + HTML
Deploy: Docker

## Database

SQLite in memory database

```
cargo install sqlx-cli
sqlx migrate add initial
```

The above commands will install SQLx CLI tool, make a migration, make a new file called <timestamp>initial.sql for your database schema.

### To embed your migrations in your application binary

On startup, after creating your database connection or pool, add:
`sqlx::migrate!().run(<&your_pool OR &mut your_connection>).await?;`

Note that the compiler won't pick up new migrations if no Rust source files have changed.
You can create a Cargo build script to work around this with `sqlx migrate build-script`.

## Run Tests

```
cargo test
```

## JWT Token expiry time leeway

Since validating time fields is always a bit tricky due to clock skew, you can add some leeway to the iat, exp and nbf validation by setting the leeway field.

## Run Development

Enable hot reload:

```
cargo watch -q -c -w src/ -x run

cargo watch -q -c -w tests/ -x "test -q test_hello -- --nocapture"
```

## Run API Client

Install and launch the Slumber API Client from https://github.com/LucasPickering/slumber

```
slumber
```

Then go through the API commands one by one to verify the APIs work properly

## Bcrypt Hash Time

Set bcrypt hash cost to 14 or above to ensure enought time cost against hackers

## Ethereum Dependencies

- Clap: A full featured, fast Command Line Argument Parser
- Eyre: Flexible concrete Error Reporting type built on std::error::Error with customizable Reports
- Hex: Encoding and decoding data into/from hexadecimal representation.

## Install Rust and global dependencies

`cargo install cargo-watch`

## Implement .env file from env.template

Example:
DB_POSTGRES_URL=protocol://username:password@host/database
[Note] MNEMONIC MUST include double quotation marks or the MNEMONIC cannot be completely read properly

```
DATABASE_URL_EXAMPLE=protocol://username:password@host/database
DB_MYSQL_URL=
DB_POSTGRES_URL=
JWT_SECRET=
HASHCOST=
ETHERSCAN_KEY=
MNEMONIC=
INFURA_KEY=
NETWORK=sepolia
MORALIS_KEY=
PRIVATE_KEY0=
ETH_ADDR0=
ETH_ADDR1=
ETH_ADDR2=0x
ETH_ADDR3=0x
ETH_ERC20TOKEN=
ETH_ERC20TOKEN_OWNER=
CHAINLINK_AGGR_V3_BTCUSD=
CHAINLINK_AGGR_V3_ETHUSD=
```

## Start Database

```
docker compose up -d --wait

docker compose logs database
```

## Generate Sea-orm entities

https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli/

[Note] Generate entity files if you change your database structure!

```
cargo install sea-orm-cli
sea-orm-cli generate entity -o src/entities -u YOUR_DATABASE_URI -o entity/src
```

## Install Ganache-CLI

https://github.com/trufflesuite/ganache

```
npm install ganache --global
```

## To Run in development mode

```bash
cargo watch -x run
```

## To Run in production mode

```bash
cargo run
```

## Make Docs for local dependency doc access

`cargo doc --open`
then open the generated html in a browser
