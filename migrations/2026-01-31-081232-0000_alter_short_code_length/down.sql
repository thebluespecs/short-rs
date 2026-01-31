-- Revert to original short_code length
ALTER TABLE urls ALTER COLUMN short_code TYPE VARCHAR(20);
