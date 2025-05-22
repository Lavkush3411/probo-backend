-- Add up migration script here
ALTER TABLE users
ADD COLUMN balance INTEGER NOT NULL DEFAULT 0;

ALTER TABLE users
ADD CONSTRAINT chk_balance_positive CHECK (balance >= 0);