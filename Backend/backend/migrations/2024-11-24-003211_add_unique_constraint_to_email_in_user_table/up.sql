-- Your SQL goes here
ALTER TABLE users ADD CONSTRAINT unique_email UNIQUE (email);