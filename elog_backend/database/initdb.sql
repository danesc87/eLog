---------------------------
-- eLOG DATABASE SCRIPTS --
---------------------------

CREATE TABLE IF NOT EXISTS user_role (
  id TINYINT NOT NULL,
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

CREATE TABLE IF NOT EXISTS pay_type (
  id TINYINT AUTO_INCREMENT,
  name VARCHAR(100) NOT NULL,
  description VARCHAR(255),
  PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS user_pay_method (
  id TINYINT NOT NULL AUTO_INCREMENT,
  user_id SMALLINT NOT NULL,
  pay_type_id TINYINT NOT NULL,
  bank_name VARCHAR(255) NOT NULL,
  description VARCHAR(255),
  enabled BOOLEAN NOT NULL DEFAULT 1,
  register_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (id),
  FOREIGN KEY (user_id) REFERENCES app_user (id),
  FOREIGN KEY (pay_type_id) REFERENCES pay_type (id)
);

CREATE TABLE IF NOT EXISTS expense (
  id INT NOT NULL AUTO_INCREMENT,
  user_pay_method_id TINYINT NOT NULL,
  ammount DOUBLE(19,6),
  description VARCHAR(255),
  register_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (id),
  FOREIGN KEY (user_pay_method_id) REFERENCES user_pay_method (id)
);


CREATE TABLE IF NOT EXISTS invalid_tokens (
  string_token TEXT NOT NULL UNIQUE,
  expiration_date TIMESTAMP NOT NULL
);
