CREATE TYPE clean_states AS ENUM ('CLEANED', 'USED', 'DIRTY', 'OUT_OF_ORDER');

CREATE TABLE
    water_closets (
        id SERIAL PRIMARY KEY,
        group_id INTEGER NOT NULL REFERENCES groups (id),
        is_disabled BOOLEAN NOT NULL DEFAULT false,
        is_available BOOLEAN NOT NULL DEFAULT true,
        is_door_opened BOOLEAN NOT NULL DEFAULT false,
        is_door_locked BOOLEAN NOT NULL DEFAULT false,
        clean_state clean_states NOT NULL
    );

INSERT INTO
    water_closets
VALUES
    (1, 1, true, true, true, false, 'CLEANED');