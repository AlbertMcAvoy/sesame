CREATE TYPE states AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');

CREATE TYPE topics AS ENUM (
    'DOOR',
    'TOILET',
    'SUPPLY',
    'CLEANLINESS',
    'OTHER'
);

CREATE TABLE
    reports (
        id SERIAL PRIMARY KEY,
        user_id INTEGER NOT NULL REFERENCES users (id),
        water_closet_id INTEGER NOT NULL REFERENCES water_closets (id),
        datetime TIMESTAMP NOT NULL,
        state states NOT NULL default 'TODO',
        topic topics NOT NULL,
        comment TEXT NOT NULL
    );

INSERT INTO
    reports
VALUES
    (
        1,
        1,
        1,
        '2024-07-16 14:35:35',
        'TODO',
        'DOOR',
        'La porte des toilettes ne voulait plus s''ouvrir via l''application, j'' ai dû l''ouvrir manuellement.'
    );