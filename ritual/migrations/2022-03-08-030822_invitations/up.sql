-- Your SQL goes here
CREATE TABLE invitations (
  id UUID NOT NULL PRIMARY KEY,
  sender_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  recipient_email VARCHAR(100) NOT NULL,
  expires_at TIMESTAMP NOT NULL
);
