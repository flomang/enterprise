-- Your SQL goes here
CREATE TABLE ritual_times (
  id SERIAL PRIMARY KEY,
  ritual_id INTEGER NOT NULL REFERENCES rituals(id),
  created_on TIMESTAMP NOT NULL
);