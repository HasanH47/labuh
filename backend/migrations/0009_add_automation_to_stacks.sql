-- Add automation fields to stacks table
ALTER TABLE stacks ADD COLUMN cron_schedule TEXT;
ALTER TABLE stacks ADD COLUMN health_check_path TEXT;
ALTER TABLE stacks ADD COLUMN health_check_interval INTEGER NOT NULL DEFAULT 30;
ALTER TABLE stacks ADD COLUMN last_stable_images TEXT;
