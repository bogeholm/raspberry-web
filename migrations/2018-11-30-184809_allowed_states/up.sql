-- Your SQL goes here
CREATE TABLE allowed_states (
    state_id INTEGER NOT NULL UNIQUE PRIMARY KEY,
    state_type TEXT NOT NULL,
    input INTEGER NOT NULL DEFAULT 0,
	output INTEGER NOT NULL DEFAULT 0,
    high INTEGER NOT NULL DEFAULT 0,
    low INTEGER NOT NULL DEFAULT 0
);

INSERT INTO allowed_states (
    state_id, 
    state_type,
    input,
    output,
    high,
    low
)
VALUES 
    (1, "mode", 1, 1, 0, 0),
    (2, "level", 0, 0, 1, 1);