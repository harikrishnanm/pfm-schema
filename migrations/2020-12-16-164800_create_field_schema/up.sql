-- Your SQL goes here

CREATE TABLE field_schema (
    id SERIAL NOT NULL PRIMARY KEY,
    field_name VARCHAR(64) NOT NULL,
    is_array BOOLEAN NOT NULL,
    is_required BOOLEAN NOT NULL,
    object_id INTEGER,
    parent INTEGER,
    pattern VARCHAR(255),
    size INTEGER,
    field_type VARCHAR(255)
);
