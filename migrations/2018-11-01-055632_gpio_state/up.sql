-- Your SQL goes here
CREATE TABLE gpio_state (
	gpio_id	INTEGER NOT NULL UNIQUE PRIMARY KEY,
    in_use	INTEGER NOT NULL DEFAULT 0,
	gpio_mode  	TEXT,
	gpio_level	TEXT,
	last_change	TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
-- Based on https://www.raspberrypi.org/documentation/usage/gpio/ & https://pinout.xyz/#
INSERT INTO gpio_state (
    gpio_id,
    in_use
)
VALUES 
    (0, 0),
    (1, 0),
    (2, 0),
    (3, 0),
    (4, 0),
    (5, 0),
    (6, 0),
    (7, 0),
    (8, 0),
    (9, 0),
    (10, 0),
    (11, 0),
    (12, 0),
    (13, 0),
    (14, 0),
    (15, 0),
    (16, 0),
    (17, 0),
    (18, 0),
    (19, 0),
    (20, 0),
    (21, 0),
    (22, 0),
    (23, 0),
    (24, 0),
    (25, 0),
    (26, 0),
    (27, 0);