-- Add down migration script here
ALTER TABLE trades
DROP COLUMN favour_price;

ALTER TABLE trades
DROP COLUMN against_price;