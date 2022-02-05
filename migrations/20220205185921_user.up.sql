-- Add up migration script here
CREATE TABLE user_table (
  id serial NOT NULL PRIMARY KEY,
  name VARCHAR(50) NOT NULL,
  age INT NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL
);
