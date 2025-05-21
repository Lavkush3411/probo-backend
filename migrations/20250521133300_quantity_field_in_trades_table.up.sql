-- Add up migration script here
ALTER TABLE trades
ADD COLUMN quantity INTEGER NOT NULL;