CREATE TABLE orders (
  id UUID NOT NULL PRIMARY KEY,
  user_id UUID NOT NULL,
  price decimal,
  qty decimal,
  typ VARCHAR(8) NOT NULL,
  side VARCHAR(8) NOT NULL,
  status VARCHAR(16) NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL
);