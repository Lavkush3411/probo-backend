-- Add up migration script here
ALTER TABLE trades ADD CONSTRAINT chk_favour_price_positive CHECK (favour_price > 50);

ALTER TABLE trades ADD CONSTRAINT chk_against_price_positive CHECK (against_price > 50);

ALTER TABLE trades ADD CONSTRAINT chk_combined_price CHECK (favour_price + against_price >= 1000);