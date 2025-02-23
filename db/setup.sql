DROP TABLE IF EXISTS events CASCADE;

CREATE TABLE events (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    type TEXT NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    data JSONB
);

CREATE INDEX idx_events_data_jsonb ON events USING GIN (data);

CREATE INDEX idx_events_user_id_id
  ON events (user_id, id);

DROP TABLE IF EXISTS states CASCADE;
CREATE TABLE IF NOT EXISTS states (
    user_id BIGINT NOT NULL,
    after_event_id BIGINT NOT NULL REFERENCES events(id),
    state JSONB,
    PRIMARY KEY (user_id, after_event_id)
);

CREATE INDEX IF NOT EXISTS idx_states_user_after_event_desc
    ON states (user_id, after_event_id DESC);

CREATE INDEX IF NOT EXISTS idx_state_jsonb
    ON states
    USING GIN (state);

