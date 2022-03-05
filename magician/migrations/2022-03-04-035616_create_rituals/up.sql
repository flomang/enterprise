-- Your SQL goes here
CREATE TABLE rituals (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f',
  created_on TIMESTAMP NOT NULL,
  updated_on TIMESTAMP NOT NULL
);
