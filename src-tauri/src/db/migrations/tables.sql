CREATE TYPE program AS (
    path VARCHAR(150),
    name VARCHAR(100),
    visible BOOLEAN,
    favourite BOOLEAN
);
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    programs program[],
    defaultBrowser VARCHAR(50)
);

