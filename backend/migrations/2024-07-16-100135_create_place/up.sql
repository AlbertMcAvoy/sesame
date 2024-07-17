CREATE TABLE
    places (
        id SERIAL PRIMARY KEY,
        group_id INTEGER NOT NULL REFERENCES groups (id),
        coordonates POINT NOT NULL
    );

INSERT INTO
    places
VALUES
    (1, 1, POINT (-118.4079, 33.9434));