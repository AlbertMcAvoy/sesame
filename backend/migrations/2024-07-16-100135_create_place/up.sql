CREATE TABLE
    places (
        id SERIAL PRIMARY KEY,
        group_id INTEGER NOT NULL REFERENCES groups (id),
        coordonates VARCHAR NOT NULL
    );

INSERT INTO
    places
VALUES
    (1, 1, '45.76165702581564, 4.85705057753092');