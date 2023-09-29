CREATE TABLE fills (
  id UUID NOT NULL PRIMARY KEY,
  order_id UUID NOT NULL REFERENCES orders(id),
  price decimal NOT NULL,
  quantity decimal NOT NULL,
  order_type VARCHAR(8) NOT NULL,
  side VARCHAR(8) NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL
);