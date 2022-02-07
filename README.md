# [Demo] Simple REST API Server powerd by Axum & sqlx & Postgres

## Use Crate's

- **Tokio (1.15.0)**
- **Axum (0.4.5)**
- **sqlx (0.5.10)**

## Setup Environment

**Build Postgres container & Create Database schema**

```sh
git clone git@github.com:Ryusei-0407/axum-rest.git

cd axum-rest

docker-compose up -d

# use sqlx -> cargo install sqlx-cli

sqlx db create

# use uuid_random_v4() -> execute postgres/uuid.sql

sqlx migrate run

make
```

**Clean Up**

```sh
sqlx db drop

cargo clean

docker-compose down
```
