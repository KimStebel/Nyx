-- Drop the table if it already exists (optional, for a clean slate)
DROP TABLE IF EXISTS events CASCADE;

-- Create the events table
CREATE TABLE events (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    type TEXT NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    data JSONB
);

-- (Optional) Create an index on the JSONB column for faster querying
CREATE INDEX idx_events_data_jsonb ON events USING GIN (data);

CREATE INDEX idx_events_user_id_id
  ON events (user_id, id);