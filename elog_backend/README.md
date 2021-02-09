# eLog Backend

This repo contains the REST backend for **eLog** expenses manager. It's built upon Rust, Actix, Diesel and MariaDB, but it can work with SQLite3 and PostgreSQL as well.

### Requirements
- Rust
- MariaDB
- Development Requirements
  - MariaDB/Mysql
    - libmariadb-devel (openSUSE)
    - libmariadb-dev-compat (Debian/Ubuntu)
  - SQLite
    - sqlite3-devel (openSUSE)
    - libsqlite3-dev (Debian/Ubuntu)

- Env File

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
GRANT ALL PRIVILEGES ON {db_name}.* TO "root"@"localhost" IDENTIFIED BY "1234abcd";
```


### Env file

`.env` file should be as follows:

```
# Server Things
SERVER_IP=0.0.0.0
SERVER_PORT=8090
RUST_LOG=actix_web=debug

# DB Data
DB_URL=mysql://user:pass@ip:port/db_name
POOL_SIZE=6

# Token
JWT_SECRET=elog-super-secret-key
TOKEN_DURATION_MIN=60
```

### Release

Release version should be changed because it'll be a hardcoded String in `session_properties.rs` file

### Run

Run eLog Backend

```bash
cargo run
```

### Build

Build eLog Backend with **debug** compatibility

```bash
cargo build
```

### Endpoints

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
path: /pay_type
method: POST
headers: Bearer token
```

Body

```json
{
	"name": "Credit Card",
	"description": "Credit Card"
}
```

#### get_pay_types

Request

```
path: /pay_type
method: GET
headers: Bearer token
```

Response

```json
[
  {
    "id": 1,
    "name": "Credit Card",
    "description": "Credit Card"
  }
]
```

#### insert_user_pay_method

Request

```
path: /user_pay_method/{pay_type_id}
method: POST
headers: Bearer token
```

Body

```json
{
	"bank_name": "Bank Name",
	"description": "Pay method description",
	"enabled": true
}
```

#### get_user_pay_methods

Request

```
path: /user_pay_method
method: GET
headers: Bearer token
```

Response

```json
[
  {
    "id": 5,
    "user_id": 1,
    "pay_type_id": 1,
    "bank_name": "Bank Name",
    "description": "Pay method description",
    "enabled": true,
    "register_date": "2020-11-24T00:56:36"
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
    "user_id": 1,
    "category": "Category Name",
    "description": "Category Description"
  }
]
```

#### insert_expense

Request

```
path: /expense/{user_category_id}/{user_pay_method_id}
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
    "user_category": "Alcohol",
    "user_pay_method": "Bank",
    "amount": 40.6,
    "description": "Whisky",
    "register_date": "2020-12-01T23:45:03"
  },
  {
    "id": 19,
    "user_category": "Supermarket",
    "user_pay_method": "Credit Card",
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

Response

```json
[
  {
    "amount": 30.08,
    "category": "Supermarket"
  },
  {
    "amount": 24.0,
    "category": "Vet"
  }
]
```
