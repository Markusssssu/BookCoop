CREATE TABLE IF NOT EXISTS Authors (
    author_id SERIAL PRIMARY KEY,
    full_name VARCHAR(255) NOT NULL,
    date_of_birth DATE,
    biography TEXT
);

INSERT INTO Authors (full_name, date_of_birth, biography) VALUES
('Джордж Оруэлл', '1903-06-25', 'Эрик Артур Блэр, более известный под псевдонимом Джордж Оруэлл, был английским писателем, журналистом и критиком.'),
('Джейн Остин', '1775-12-16', 'Английская романистка, чьи работы критикуют и исследуют британскую знать в конце 18 века.');
