-- Add up migration script here
ALTER TABLE contact
ALTER COLUMN mobile_number TYPE VARCHAR(15);