CREATE TABLE goals (
    id UUID NOT NULL PRIMARY KEY,
    ritual_id UUID NOT NULL REFERENCES rituals(id) ON DELETE CASCADE,
    interval_minutes INTEGER,
    started_at TIMESTAMP NOT NULL,
    ended_at TIMESTAMP,
    status TEXT,
    emojii_url TEXT,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);