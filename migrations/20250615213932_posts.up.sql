-- Add up migration script here
CREATE TABLE IF NOT EXISTS posts (
    id TEXT PRIMARY KEY NOT NULL,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP
);