

-- Create table to log balance changes
CREATE TABLE IF NOT EXISTS user_balance_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    user_id VARCHAR(255) NOT NULL REFERENCES users(id),
    old_balance INTEGER NOT NULL,
    new_balance INTEGER NOT NULL
);

-- Create the function that logs balance changes
CREATE OR REPLACE FUNCTION on_balance_change()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.balance IS DISTINCT FROM OLD.balance THEN
        INSERT INTO user_balance_logs(user_id, old_balance, new_balance, created_at)
        VALUES (OLD.id, OLD.balance, NEW.balance, NOW());
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create the trigger
CREATE TRIGGER trigger_on_balance_change
AFTER UPDATE ON users
FOR EACH ROW
WHEN (OLD.balance IS DISTINCT FROM NEW.balance)
EXECUTE FUNCTION on_balance_change();
