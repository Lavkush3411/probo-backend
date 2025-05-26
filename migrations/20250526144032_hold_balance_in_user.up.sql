-- Add up migration script here
ALTER TABLE users
ADD COLUMN hold_balance INTEGER NOT NULL DEFAULT 0;

ALTER TABLE users ADD CONSTRAINT check_hold_balance_positive CHECK (hold_balance >= 0);