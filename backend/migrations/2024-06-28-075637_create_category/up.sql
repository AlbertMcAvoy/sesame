-- Your SQL goes here

CREATE TABLE categories (
    category_id uuid DEFAULT gen_random_uuid(),
    name VARCHAR NOT NULL,
    PRIMARY KEY (category_id) 
);