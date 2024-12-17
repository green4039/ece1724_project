-- Your SQL goes here
CREATE TABLE accounts (
    account_id SERIAL PRIMARY KEY ,
    email TEXT NOT NULL REFERENCES users(email) ON DELETE CASCADE,
    type TEXT NOT NULL
);