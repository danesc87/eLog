# eLog Backend

This repo contains the REST backend for **eLog** expenses manager. It's built upon Rust, Actix, Diesel and MariaDB, but it can work with SQLite3 and PostgreSQL as well.

## Requirements

```
- Rust
- MariaDB
- Development Requirements
  - MariaDB/Mysql
    - libmariadb-devel (openSUSE)
    - libmariadb-dev-compat (Debian/Ubuntu)
  - SQLite
    - sqlite3-devel (openSUSE)
    - libsqlite3-dev (Debian/Ubuntu)

- Config File
```

### Init MariaDB

Running **MariaDB** inside docker

```bash
docker run --name mariadb -e MYSQL_ROOT_PASSWORD={password} -p 3306:3306 -d mariadb:10.5-focal
```

Create Database

```bash
docker exec -it mariadb bash
mysql -h 127.0.0.1 -u root -p{password}
```

```mysql
CREATE DATABASE {db_name} DEFAULT CHARACTER SET utf8 COLLATE utf8_unicode_ci;
GRANT ALL PRIVILEGES ON {db_name}.* TO "{user_name}"@"{db_host}" IDENTIFIED BY "{password}";
```

> `{password}, {db_name}, {user_name} and {db_host}`  
> are variables and they should be replaced with your own i.e.  
> `{password} -> 1234abcd`  
> `{db_name} -> dev_database`  
> `{user_name} -> dev_user`  
> `{db_host} -> localhost`

### Config file

**eLog** config file must be called `elog.yml` and be placed on the same file path as *elog_backend* binary, this config file should be as follows:

```yml
# Ip address of eLog backend 0.0.0.0 means can accept connections from everywhere
ip_address: 0.0.0.0
# Port where eLog backend will listen
server_port: 8090
# Log type and level could be something like:
# ERROR, WARN, INFO, DEBUG, TRACE
# or have specified a library like:
# actix_web=DEBUG, actix_web=INFO
log_type: DEBUG
# Database config
database:
  # URL of database for eLog, should contain:
  # user, pass, ip address, port and db schema
  db_url: mysql://user:pass@ip:port/db_name
  # Connection pool allow maximum number of connections managed by the pool
  pool_size: 6
# Token config
token:
  # Super secret key for encoding tokens
  jwt_secret: elog-super-secret-key
  # Duration in minutes
  duration: 60
```

> According to samples below `DB_URL` will be something like this:  
> `mysql:://dev_user:1234abcd@127.0.0.1:3306/dev_database`

You can find a sample version called `elog-sample.yml` in this repo.

## Release

Release version should be changed because it'll be a hardcoded String in `session_properties.rs` file

## Run

Run eLog Backend

```bash
cargo run
```

## Build

Build eLog Backend with **debug** compatibility

```bash
cargo build
```

## Endpoints

This has a sample of the current endpoints and how they work

#### register

Request

```
path: /register
method: POST
```

Body

```json
{
	"first_name": "Jon",
	"last_name": "Doe",
	"username": "jd",
	"email": "jd@test.rs",
	"password": "1234abcd"
}
```

#### login

Request

```
path: /login
method: POST
```
Body

```json
{
	"username": "dc",
	"password": "1234abcd"
}
```

#### logout

Request

```
path: /logout
method: GET
headers: Bearer token
```

Response

```
Only 200 without body
```

#### session_properties

Request

```
path: /session_properties
method: GET
headers: Bearer token or not Token
```

Response

Always Ok with following body

```
{
  "is_valid_token": false,
  "properties": null
}
```
> `properties` will be null if no token was provided or token is expired.
> only will have data if token is valid and user has properties

#### insert_pay_type

Request

```
path: /user_pay_type
method: POST
headers: Bearer token
```

Body

```json
{
	"name": "Credit Card",
	"bank_name": "Awesome Bank",
	"description": "Credit Card"
}
```

#### get_pay_types

Request

```
path: /user_pay_type
method: GET
headers: Bearer token
```

Response

```json
[
  {
    "id": 1,
    "name": "Credit Card",
    "bank_name": "Awesome Bank",
    "description": "Credit Card"
  }
]
```

#### insert_user_category

Request

```
path: /user_category
method: POST
headers: Bearer token
```

Body

```json
{
  "category": "Category Name",
  "description": "Category Description"
}
```

#### get_user_categories

Request

```
path: /user_category
method: GET
headers: Bearer token
```

Response

```json
[
  {
    "id": 1,
    "category": "Category Name",
    "description": "Category Description"
  }
]
```

#### insert_expense

Request

```
path: /expense/{user_pay_type_id}/{user_category_id}
method: POST
headers: Bearer token
```

Body

```json
{
	"amount": 40.6,
	"description": "Whisky"
}
```

#### get_expenses

Request

```
path: /expense
method: POST
headers: Bearer token
```

Response

```json
[
  {
    "id": 18,
    "user_pay_type": "Credit Card",
    "user_category": "Alcohol",
    "amount": 40.6,
    "description": "Whisky",
    "register_date": "2020-12-01T23:45:03"
  },
  {
    "id": 19,
    "user_pay_type": "Debit Card",
    "user_category": "Supermarket",
    "amount": 63.07,
    "description": "Supermarket",
    "register_date": "2020-12-01T23:45:47"
  }
]
```

#### get_expenses_for_report

Request

```
path: /report/expense
method: GET
headers: Bearer token
```

Query Parameters

```
since_when -> Milliseconds
until_when -> Milliseconds
```

Response

```json
{
  "expenses": [
    {
      "amount": 30.08,
      "category": "Supermarket"
    },
    {
      "amount": 24.0,
      "category": "Vet"
    }
  ],
  "since_when": "1970-01-01T00:00:00",
  "until_when": "2021-02-06T04:03:03"
}
```
