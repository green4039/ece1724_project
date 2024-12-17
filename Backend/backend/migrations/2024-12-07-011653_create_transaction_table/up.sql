-- Your SQL goes here
CREATE TABLE transactions (
    trans_id SERIAL PRIMARY KEY,
    email TEXT NOT NULL REFERENCES users(email) ON DELETE CASCADE,
    category_id INT NOT NULL REFERENCES categories(category_id) ON DELETE CASCADE,
    amount FLOAT NOT NULL,
    notes TEXT
);
