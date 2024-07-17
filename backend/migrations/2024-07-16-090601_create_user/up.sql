CREATE TYPE roles AS ENUM ('USER', 'CLIENT', 'WORKER', 'ADMIN');

CREATE TABLE
    users (
        id SERIAL PRIMARY KEY,
        mail VARCHAR NOT NULL,
        phone VARCHAR NOT NULL,
        role roles NOT NULL
    );

INSERT INTO
    users
VALUES
    (
        1,
        'pellegrinpierre69510@gmail.com',
        '0623938154',
        'ADMIN'
    );