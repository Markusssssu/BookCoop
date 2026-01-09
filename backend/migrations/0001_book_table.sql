CREATE TABLE IF NOT EXISTS book (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    isbn VARCHAR(13) UNIQUE, -- ISBN-13 usually has 13 digits and can contain 'X' in ISBN-10, so VARCHAR is appropriate
    published DATE,
    admin_id INTEGER NOT NULL
);