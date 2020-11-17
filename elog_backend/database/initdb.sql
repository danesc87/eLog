-- eLOG DATABASE SCRIPTS

CREATE TABLE IF NOT EXISTS user_role (
  id tinyint not null,
  description varchar(25),
  primary key(id)
);

CREATE TABLE IF NOT EXISTS app_user (
  id int not null auto_increment,
  first_name varchar(100) not null,
  last_name varchar(100) not null,
  username varchar(100) not null unique,
  email varchar(255) not null,
  password varchar(255) not null,
  role_id tinyint,
  status boolean not null default 1,
  register_date timestamp not null default CURRENT_TIMESTAMP,
  primary key (id),
  foreign key (role_id) references user_role(id)
);

CREATE TABLE IF NOT EXISTS pay_type (
  id tinyint auto_increment,
  name varchar(100) not null,
  description varchar(255),
  status boolean not null default 1,
  primary key(id)
);

CREATE TABLE IF NOT EXISTS user_pay_method (
  id int not null auto_increment,
  user_id int,
  location varchar(100),
  bank_name varchar(255) not null,
  description varchar(255),
  status boolean not null default 1,
  register_date timestamp not null default CURRENT_TIMESTAMP,
  primary key(id),
  foreign key(user_id) references app_user(id)
);

CREATE TABLE IF NOT EXISTS pay_method_rule (
  id int auto_increment,
  user_pay_method_id int not null,
  rule_description varchar(255),
  frecuency varchar(10) not null,
  rule_min_quota decimal(19,6),
  rule_max_quota decimal(19,6),
  status boolean not null default 1,
  primary key(id),
  foreign key(user_pay_method_id) references user_pay_method(id)
);

CREATE TABLE IF NOT EXISTS expense (
  id int not null auto_increment,
  user_pay_method_id int not null,
  expense_value decimal(19,6),
  register_date timestamp not null default CURRENT_TIMESTAMP,
  primary key(id),
  foreign key(user_pay_method_id) references user_pay_method (id)
);
