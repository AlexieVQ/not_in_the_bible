ALTER TABLE requests ADD COLUMN quoted BOOLEAN NOT NULL DEFAULT 'f';
ALTER TABLE responses ADD COLUMN quoted BOOLEAN NOT NULL DEFAULT 'f';