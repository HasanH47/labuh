-- Add proxied field to domains
ALTER TABLE domains ADD COLUMN proxied BOOLEAN NOT NULL DEFAULT 0;
