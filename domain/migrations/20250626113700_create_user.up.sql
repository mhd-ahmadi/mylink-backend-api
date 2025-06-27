-- Add up migration script here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(128) NOT NULL,
    password_hash VARCHAR(128) NOT NULL,
    created_on_utc TIMESTAMPTZ NOT NULL DEFAULT NOW()
);