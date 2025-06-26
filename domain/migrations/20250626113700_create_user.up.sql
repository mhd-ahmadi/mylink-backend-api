-- Add up migration script here
CREATE TABLE users(
    id serial primary key,
    username Varchar(128),
    password_hash Varchar(128)
)