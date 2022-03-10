-- Your SQL goes here
CREATE TABLE ritual_times (
  id UUID NOT NULL PRIMARY KEY,
  ritual_id UUID NOT NULL REFERENCES rituals(id) ON DELETE CASCADE,
  created_at TIMESTAMP NOT NULL
);