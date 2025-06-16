-- Add up migration script here
ALTER TABLE users
ADD COLUMN role VARCHAR(50) NOT NULL DEFAULT 'User';