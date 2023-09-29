CREATE TABLE goals (
    id UUID NOT NULL PRIMARY KEY,
    ritual_id UUID NOT NULL REFERENCES rituals(id) ON DELETE CASCADE,
    interval_minutes INTEGER,
    status TEXT,
    emojii_url TEXT,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TABLE acheivements (
    id UUID NOT NULL PRIMARY KEY,
    goal_id UUID NOT NULL REFERENCES goals(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);