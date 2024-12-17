-- Your SQL goes here
CREATE TABLE categories (
    category_id SERIAL PRIMARY KEY,
    email TEXT NOT NULL REFERENCES users(email) ON DELETE CASCADE,
    nickname TEXT NOT NULL,
    category_type TEXT NOT NULL,
    budget FLOAT NOT NULL,
    budget_freq TEXT NOT NULL
);
