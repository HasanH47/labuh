-- Phase 12: Multi-User & Teams

-- 1. Create teams table
CREATE TABLE IF NOT EXISTS teams (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- 2. Create team_members table
CREATE TABLE IF NOT EXISTS team_members (
    team_id TEXT NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role TEXT NOT NULL, -- OWNER, ADMIN, DEVELOPER, VIEWER
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (team_id, user_id)
);

-- 3. Add team_id to stacks
ALTER TABLE stacks ADD COLUMN team_id TEXT REFERENCES teams(id);

-- 4. Add team_id to registry_credentials
ALTER TABLE registry_credentials ADD COLUMN team_id TEXT REFERENCES teams(id);

-- 5. Data Migration: Create a Personal team for each user and migrate their resources
-- We use user_id as team_id for their initial "Personal Team" to simplify the migration.
INSERT INTO teams (id, name, created_at, updated_at)
SELECT id, 'Personal Team', created_at, updated_at FROM users;

INSERT INTO team_members (team_id, user_id, role, created_at, updated_at)
SELECT id, id, 'OWNER', created_at, updated_at FROM users;

UPDATE stacks SET team_id = user_id;
UPDATE registry_credentials SET team_id = user_id;

-- 6. Create indexes
CREATE INDEX IF NOT EXISTS idx_team_members_user_id ON team_members(user_id);
CREATE INDEX IF NOT EXISTS idx_stacks_team_id ON stacks(team_id);
CREATE INDEX IF NOT EXISTS idx_registry_credentials_team_id ON registry_credentials(team_id);
