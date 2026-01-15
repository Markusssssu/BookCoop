CREATE TABLE IF NOT EXISTS Admin (
    admin_id SERIAL PRIMARY KEY,
    full_name VARCHAR(255) NOT NULL,
    keyword VARCHAR(255) NOT NULL
    );

INSERT INTO Admin (full_name, keyword) VALUES
('Марк Мансуров Радикович', 'ГОЙДА'),
('Анастасия Барьева Витальевна', 'ХОРМА');
