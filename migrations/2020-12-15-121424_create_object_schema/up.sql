-- Your SQL goes here

CREATE TABLE object_schema(
    id SERIAL PRIMARY KEY,
    namespace VARCHAR(50) UNIQUE NOT NULL,
    object_name VARCHAR(50) UNIQUE NOT NULL,
    object_url VARCHAR(50) UNIQUE NOT NULL
)
