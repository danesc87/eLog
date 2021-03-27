---------------------------
-- eLOG DATABASE SCRIPTS --
---------------------------

CREATE TABLE IF NOT EXISTS user_role (
  id SMALLINT NOT NULL,
  description VARCHAR(25),
  PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS app_user (
  id SMALLINT NOT NULL AUTO_INCREMENT,
  first_name VARCHAR(100) NOT NULL,
  last_name VARCHAR(100) NOT NULL,
  username VARCHAR(100) NOT NULL UNIQUE,
  email VARCHAR(255) NOT NULL,
  password VARCHAR(255) NOT NULL,
  register_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS user_pay_type (
  id SMALLINT AUTO_INCREMENT,
  user_id SMALLINT NOT NULL,
  name VARCHAR(100) NOT NULL,
  bank_name VARCHAR(255),
  description VARCHAR(255),
  PRIMARY KEY (id),
  FOREIGN KEY (user_id) REFERENCES app_user (id)
);

CREATE TABLE IF NOT EXISTS user_category (
  id SMALLINT NOT NULL AUTO_INCREMENT,
  user_id SMALLINT NOT NULL,
  category VARCHAR(100) NOT NULL,
  description VARCHAR(255),
  PRIMARY KEY(id),
  FOREIGN KEY (user_id) REFERENCES app_user (id)
);

CREATE TABLE IF NOT EXISTS expense (
  id INT NOT NULL AUTO_INCREMENT,
  user_pay_type_id SMALLINT NOT NULL,
  user_category_id SMALLINT NOT NULL,
  amount DOUBLE(19,6),
  description VARCHAR(255),
  register_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (id),
  FOREIGN KEY (user_category_id) REFERENCES user_category (id),
  FOREIGN KEY (user_pay_type_id) REFERENCES user_pay_type (id)
);


CREATE TABLE IF NOT EXISTS invalid_tokens (
  string_token TEXT NOT NULL UNIQUE,
  expiration_date TIMESTAMP NOT NULL
);
