# [Demo] Simple REST API Server powerd by Axum & sqlx & Postgres

## Use Crate's

- **Tokio (1.15.0)**
- **Axum (0.4.5)**
- **sqlx (0.5.10)**

## Setup Environment

**Build Postgres container & Create Database schema**

```sh
# if unused sqlx-cli
# cargo install sqlx-cli

make setup
```

**Clean Up**

```sh
make clean
```

**Create Table Schema**

```sh
sqlx migrate run
```

**Revert Database table**

```sh
sqlx migrate revert
```
