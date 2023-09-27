CREATE TABLE roles (
    id serial primary key,
    name varchar(64) not null unique
);

-- https://www.postgresql.org/docs/current/uuid-ossp.html
-- so we can use uuid_generate_v4() function
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    role_id INTEGER REFERENCES roles(id) NOT NULL,
    username TEXT UNIQUE NOT NULL,
    first_name VARCHAR(64) NOT NULL,
    last_name VARCHAR(64) NOT NULL,
    email VARCHAR(256) UNIQUE NOT NULL,
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    hash TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- use our custom function from diesel initial setup migration to update the updated_at column
SELECT diesel_manage_updated_at('users');

-- let's add some roles
-- master can do everything
-- admin can do everything except managing roles
-- user can only read data
INSERT INTO roles (name) VALUES ('master'), ('admin'), ('user');
