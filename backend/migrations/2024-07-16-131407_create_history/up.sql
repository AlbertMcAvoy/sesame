CREATE TYPE actions AS ENUM (
    'DOOR_OPENING',
    'DOOR_CLOSING',
    'LOCK_OPENING',
    'LOCK_CLOSING',
    'QR_CODE_SCAN',
    'NFC_SCAN'
);

CREATE TABLE
    histories (
        id SERIAL PRIMARY KEY,
        water_closet_id INTEGER NOT NULL REFERENCES water_closets (id),
        datetime TIMESTAMP NOT NULL,
        action actions NOT NULL
    );

INSERT INTO
    histories
VALUES
    (
        1,
        1,
        '2024-07-16 14:30:00',
        'QR_CODE_SCAN'
    );