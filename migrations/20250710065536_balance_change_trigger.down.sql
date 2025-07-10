-- Add down migration script here
-- Drop the trigger if it exists
DROP TRIGGER IF EXISTS trigger_on_balance_change ON users;

-- Drop the trigger function
DROP FUNCTION IF EXISTS on_balance_change;

-- Drop the balance log table
DROP TABLE IF EXISTS user_balance_logs;

-- Optionally drop the uuid-ossp extension (only if you're sure nothing else depends on it)
-- DROP EXTENSION IF EXISTS "uuid-ossp";