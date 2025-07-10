-- Add up migration script here
ALTER TABLE user_balance_logs
ALTER COLUMN created_at TYPE TIMESTAMPTZ
USING created_at AT TIME ZONE 'Asia/Kolkata';