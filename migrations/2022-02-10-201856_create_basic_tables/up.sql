-- Your SQL goes here

CREATE TABLE users (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    hash BYTEA NOT NULL,
    salt VARCHAR(255) NOT NULL,
    email VARCHAR(128) NOT NULL UNIQUE,
    user_name VARCHAR(32) NOT NULL UNIQUE,
    slug VARCHAR(32) NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    role VARCHAR(32) NOT NULL DEFAULT 'user',
    validated bool NOT NULL DEFAULT false
);

CREATE UNIQUE INDEX users__email_idx ON users(email);