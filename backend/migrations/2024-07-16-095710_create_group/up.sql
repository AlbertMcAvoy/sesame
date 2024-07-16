CREATE TABLE
    groups (
        id SERIAL PRIMARY KEY,
        user_id INTEGER NOT NULL REFERENCES users (id),
        name VARCHAR NOT NULL
    );

INSERT INTO
    groups
VALUES
    (1, 1, 'LAX Airport terminal 3 #1');