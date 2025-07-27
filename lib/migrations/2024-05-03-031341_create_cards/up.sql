-- Your SQL goes here

CREATE TABLE cards (
  id integer PRIMARY KEY NOT NULL,
  title VARCHAR NOT NULL,
  image BLOB,
  price decimal
)
