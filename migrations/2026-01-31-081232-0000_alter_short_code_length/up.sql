-- Increase short_code length to accommodate temp codes during insert
ALTER TABLE urls ALTER COLUMN short_code TYPE VARCHAR(200);
