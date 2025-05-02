-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- CREATE TYPE gender_enum AS ENUM ('Male', 'Female');
-- CREATE TYPE species_enum AS ENUM ('Alien', 'Head', 'Human', 'Monster', 'Mutant', 'Robot');
-- CREATE TYPE status_enum AS ENUM ('Alive', 'Dead');

CREATE TABLE characters (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(50) NOT NULL UNIQUE,
    gender VARCHAR(10),
    species VARCHAR(10),
    status VARCHAR(10),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    image VARCHAR(100)
);

CREATE TABLE episodes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(50) NOT NULL UNIQUE,
    production_code VARCHAR(8) NOT NULL,
    season SMALLINT NOT NULL,
    number SMALLINT NOT NULL
);
