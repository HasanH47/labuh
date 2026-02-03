CREATE TABLE node_metrics (
    id TEXT PRIMARY KEY,
    cpu_percent REAL NOT NULL,
    memory_usage INTEGER NOT NULL,
    memory_total INTEGER NOT NULL,
    disk_usage INTEGER NOT NULL,
    disk_total INTEGER NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE INDEX idx_node_metrics_timestamp ON node_metrics(timestamp);

CREATE TABLE container_metrics (
    id TEXT PRIMARY KEY,
    container_id TEXT NOT NULL,
    stack_id TEXT NOT NULL,
    cpu_percent REAL NOT NULL,
    memory_usage INTEGER NOT NULL,
    memory_limit INTEGER NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE INDEX idx_container_metrics_timestamp ON container_metrics(timestamp);
CREATE INDEX idx_container_metrics_container_id ON container_metrics(container_id);
