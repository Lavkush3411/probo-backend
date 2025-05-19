-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS users (
        id VARCHAR(255) PRIMARY KEY DEFAULT uuid_generate_v4 (),
        name VARCHAR(255) NOT NULL,
        email VARCHAR(255) NOT NULL UNIQUE,
        password VARCHAR(255) NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    );

CREATE TABLE
    IF NOT EXISTS opinions (
        id VARCHAR(255) PRIMARY KEY DEFAULT uuid_generate_v4 (),
        question VARCHAR(255) NOT NULL,
        description TEXT,
        result BOOLEAN DEFAULT NULL
    );

CREATE TABLE
    IF NOT EXISTS trades (
        id VARCHAR(255) PRIMARY KEY DEFAULT uuid_generate_v4 (),
        opinion_id VARCHAR(255) NOT NULL REFERENCES opinions (id),
        favour_user_id VARCHAR(255) NOT NULL REFERENCES users (id),
        against_user_id VARCHAR(255) NOT NULL REFERENCES users (id),
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    );