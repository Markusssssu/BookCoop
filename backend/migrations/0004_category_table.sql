CREATE TABLE category (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    parent_id BIGINT REFERENCES category(id)
);