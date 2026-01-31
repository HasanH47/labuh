-- Create container_resources table for CPU/Memory limits
CREATE TABLE IF NOT EXISTS container_resources (
    id TEXT PRIMARY KEY,
    stack_id TEXT NOT NULL REFERENCES stacks(id) ON DELETE CASCADE,
    service_name TEXT NOT NULL,
    cpu_limit REAL, -- Fractional CPUs (e.g. 0.5)
    memory_limit INTEGER, -- in Bytes
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(stack_id, service_name)
);

-- Create resource_metrics table for historical monitoring
CREATE TABLE IF NOT EXISTS resource_metrics (
    id TEXT PRIMARY KEY,
    container_id TEXT NOT NULL,
    stack_id TEXT NOT NULL REFERENCES stacks(id) ON DELETE CASCADE,
    cpu_usage REAL NOT NULL,
    memory_usage INTEGER NOT NULL,
    timestamp TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_metrics_stack_id ON resource_metrics(stack_id);
CREATE INDEX IF NOT EXISTS idx_metrics_timestamp ON resource_metrics(timestamp);
