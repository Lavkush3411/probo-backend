-- Add up migration script here
ALTER TABLE trades
ADD COLUMN favour_price INTEGER NOT NULL;

ALTER TABLE trades
ADD COLUMN against_price INTEGER NOT NULL;