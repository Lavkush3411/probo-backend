-- Add up migration script here
-- Alter the column type to TIMESTAMPTZ and interpret the existing values as UTC
ALTER TABLE user_balance_logs
ALTER COLUMN created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC';