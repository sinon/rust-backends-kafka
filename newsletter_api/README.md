# Newsletter API

Following along with the Zero to Production but using `axum` instead of `actix-web`.



```bash
cargo install sqlx-cli --no-default-features --features native-tls,postgres


docker run -p 5432:5432 --name some-postgres -e POSTGRES_PASSWORD=mysecretpassword -d postgres
DATABASE_URL=postgres://postgres:mysecretpassword@localhost:5432/postgres cargo sqlx migrate run
DATABASE_URL=postgres://postgres:mysecretpassword@localhost:5432/postgres cargo sqlx prepare

DATABASE_URL=postgres://postgres:mysecretpassword@localhost:5432/postgres cargo test
```
