-- Your SQL goes here

CREATE TABLE object_schema(
    id SERIAL PRIMARY KEY,
    object_name VARCHAR(50) UNIQUE NOT NULL,
    object_url VARCHAR(50) UNIQUE NOT NULL
)
