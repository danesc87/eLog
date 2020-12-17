# eLog Backend

This repo contains the REST backend for **eLog** expenses manager. It's built upon Rust, Actix, Diesel and MariaDB

### Requirements
- Rust
- MariaDB
- Development Requirements
  - libmariadb-devel (openSUSE)
  - libmariadb-dev-compat (Ubuntu)

### Init MariaDB

Running **MariaDB** inside docker

```bash
docker run --name mariadb -e MYSQL_ROOT_PASSWORD=1234abcd -p 3306:3306 -d mariadb:10.5-focal
```

Create Database

```bash
docker exec -it mariadb bash
mysql -h 127.0.0.1 -u root -p1234abcd
```

```mysql
CREATE DATABASE elog DEFAULT CHARACTER SET utf8 COLLATE utf8_unicode_ci;
GRANT ALL PRIVILEGES ON elog.* TO "root"@"localhost" IDENTIFIED BY "1234abcd";
```


### Run

Run eLog backend

```bash
cd elog_backend
cargo run
```
