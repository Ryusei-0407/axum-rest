# [Demo] Simple REST API Server powerd by Axum & sqlx & CockroachDB

## Use Crate's

- **Tokio (1.15.0)**
- **Axum (0.4.5)**
- **sqlx (0.5.10)**

## Setup Environment

**Start CockroachDB cluster & CockroachDB UI**

```sh
git clone git@github.com:Ryusei-0407/axum-rest.git

cd axum-rest

docker-compose up -d

# use sqlx -> cargo install sqlx-cli

sqlx db create

sqlx migrate run

make
```

visit the CockroachDB UI at http://localhost:8080

**Clean Up**

```sh
sqlx db drop

cargo clean

docker-compose down
```
