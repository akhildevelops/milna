-- Add down migration script here
ALTER TABLE contact
ALTER COLUMN mobile_number TYPE INT USING (mobile_number::integer);