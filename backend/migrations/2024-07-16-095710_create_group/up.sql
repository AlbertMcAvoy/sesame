CREATE TABLE
    groups (
        id SERIAL PRIMARY KEY,
        user_id INTEGER NOT NULL REFERENCES users (id)
    );

INSERT INTO
    groups
VALUES
    (1, 1);