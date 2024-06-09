# solid_biljard

## Developing

```bash
sqlx database create --database-url 'sqlite://dev.db'
```

```bash
sqlx migrate run --source sqlite_migrations --database-url 'sqlite://dev.db'
```

Have either bun or npm installed.

Either run #1 (sqlite) or #2 (postgres)

```bash
cargo run
```

```bash
cargo run --features postgres
```
