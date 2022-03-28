-- Your SQL goes here
CREATE TABLE ritual_moments (
  id UUID NOT NULL PRIMARY KEY,
  ritual_id UUID NOT NULL REFERENCES rituals(id) ON DELETE CASCADE,
  notes TEXT,
  created_at TIMESTAMP NOT NULL
);