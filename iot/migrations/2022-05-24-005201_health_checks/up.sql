CREATE TABLE health_checks (
  id SERIAL PRIMARY KEY,
  device_uuid UUID not null,
  data jsonb not null,
  user_id UUID not null,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)