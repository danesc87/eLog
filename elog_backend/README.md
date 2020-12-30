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
path: /user_pay_method/{user_id}/{pay_type_id}
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

#### insert_expense

Request

```
path: /expense
method: POST
headers: Bearer token
```

Body

```json
{
	"user_pay_method_id": 5,
	"ammount": 40.6,
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
    "user_pay_method_id": 5,
    "ammount": 40.6,
    "description": "Whisky",
    "register_date": "2020-12-01T23:45:03"
  },
  {
    "id": 19,
    "user_pay_method_id": 5,
    "ammount": 63.07,
    "description": "Supermarket",
    "register_date": "2020-12-01T23:45:47"
  }
]
```
