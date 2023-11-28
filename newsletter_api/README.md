```bash
docker run -p 5432:5432 --name some-postgres -e POSTGRES_PASSWORD=mysecretpassword -d postgres
DATABASE_URL=postgres://postgres:mysecretpassword@localhost:5432/postgres sqlx migrate run
DATABASE_URL=postgres://postgres:mysecretpassword@localhost:5432/postgres cargo sqlx prepare

DATABASE_URL=postgres://postgres:mysecretpassword@localhost:5432/postgres cargo test
```
