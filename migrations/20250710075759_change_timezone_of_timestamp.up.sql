-- Add up migration script here
ALTER TABLE user_balance_logs
ALTER COLUMN created_at SET DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'UTC');
