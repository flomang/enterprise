-- Your SQL goes here
CREATE TABLE users (
  id UUID NOT NULL PRIMARY KEY,
  email VARCHAR(64) NOT NULL UNIQUE,
  email_verified bool NOT NULL default false,
  username VARCHAR(48) NOT NULL UNIQUE,
  avatar_url VARCHAR(128),
  hash VARCHAR(122) NOT NULL, --argon hash
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL
);

insert into users (id, email, email_verified, username, hash, created_at, updated_at) values ('00e8da9b-9ae8-4bdd-af76-af89bed2262f','master@splinter.com', true, 'master splinter', '', now(), now());