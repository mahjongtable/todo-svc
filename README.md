
# For local, Initialise the MySQL database in docker.
```bash
docker pull mysql
docker run --name todo-service-mydb -e MYSQL_ROOT_PASSWORD=123456 -p 3306:3306 -d mysql --character-set-server=utf8mb4 --collation-server=utf8mb4_unicode_ci
```

# Run migration files.
You need to install the sqlx-cli via cargo, just like "cargo install sqlx-cli".
```bash
cargo
```