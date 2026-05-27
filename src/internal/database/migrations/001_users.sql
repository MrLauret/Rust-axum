CREATE TABLE IF NOT EXISTS Users {
    id SERIAL PRIMARY KEY,
    username varchar(50) NOT NULL,
    password_hash varchar(255) NOT NULL,
}