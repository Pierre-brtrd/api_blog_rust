-- Add up migration script here
ALTER TABLE users ADD COLUMN first_name VARCHAR(255);

ALTER TABLE users ADD COLUMN last_name VARCHAR(255);