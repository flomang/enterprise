CREATE TABLE orders (
  id UUID NOT NULL PRIMARY KEY,
  user_id UUID NOT NULL REFERENCES users(id),
  order_asset VARCHAR(8) NOT NULL,
  price_asset VARCHAR(8) NOT NULL,
  price decimal,
  quantity decimal NOT NULL,
  order_type VARCHAR(8) NOT NULL,
  side VARCHAR(8) NOT NULL,
  status VARCHAR(16) NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL
);