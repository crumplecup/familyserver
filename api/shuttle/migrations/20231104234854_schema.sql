DROP TABLE IF EXISTS families;
CREATE TABLE IF NOT EXISTS families (
    family_id serial PRIMARY KEY,
    name TEXT NOT NULL
);

DROP TABLE IF EXISTS users;
CREATE TABLE IF NOT EXISTS users (
    id uuid PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL
);

DROP TABLE IF EXISTS identity;
CREATE TABLE IF NOT EXISTS identity (
    identity_id serial PRIMARY KEY,
    -- FOREIGN KEY user_id REFERENCES users(id),
    first_name TEXT NOT NULL,
    last_name TEXT,
    middle_names TEXT
    -- FOREIGN KEY family_id REFERENCES families(family_id),
);

DROP TABLE IF EXISTS health;
CREATE TABLE IF NOT EXISTS health (
    health_id serial PRIMARY KEY,
    -- FOREIGN KEY user_id REFERENCES users(id),
    age TEXT,
    height_m TEXT,
    weight_kg TEXT
);
