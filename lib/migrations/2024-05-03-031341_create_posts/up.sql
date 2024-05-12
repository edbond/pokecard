-- Your SQL goes here

CREATE TABLE cards (
  id integer PRIMARY KEY,
  title VARCHAR NOT NULL,
  image BLOB,
  price decimal
)
