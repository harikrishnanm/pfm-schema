-- Your SQL goes here

CREATE TABLE field_schema (
    id SERIAL NOT NULL,
    field_name VARCHAR(64) NOT NULL,
    is_array BOOLEAN NOT NULL,
    is_required BOOLEAN NOT NULL,
    object_id BIGINT,
    parent BIGINT,
    pattern VARCHAR(255),
    size BIGINT,
    type VARCHAR(255)
);
