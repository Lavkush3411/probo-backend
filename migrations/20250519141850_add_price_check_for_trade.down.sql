-- Add down migration script here
ALTER TABLE trades
DROP CONSTRAINT chk_favour_price_positive;

ALTER TABLE trades
DROP CONSTRAINT chk_against_price_positive;

ALTER TABLE trades
DROP CONSTRAINT chk_combined_price;