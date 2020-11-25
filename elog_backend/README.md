# eLog Backend


### Requirements
- rust
- libmariadb-devel (devel)

### Connect to MariaDB

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
