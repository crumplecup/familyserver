CREATE TABLE IF NOT EXISTS families (
    family_id serial PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
    id serial PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS identity (
    FOREIGN KEY user_id REFERENCES users(id) PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT,
    middle_names TEXT,
    FOREIGN KEY family_id REFERENCES families(family_id),
)

CREATE TABLE IF NOT EXISTS health {
    FOREIGN KEY user_id REFERENCES users(id) PRIMARY KEY,
    age TEXT,
    height_m TEXT,
    weight_kg TEXT,
}
