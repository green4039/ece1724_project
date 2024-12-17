-- Your SQL goes here
ALTER TABLE transactions
ADD COLUMN account_id INT NOT NULL
REFERENCES accounts(account_id)
ON DELETE CASCADE;
