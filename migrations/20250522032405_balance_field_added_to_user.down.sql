-- Add down migration script here
ALTER TABLE users
DROP CONSTRAINT chk_balance_positive;

ALTER TABLE users
DROP COLUMN balance;

