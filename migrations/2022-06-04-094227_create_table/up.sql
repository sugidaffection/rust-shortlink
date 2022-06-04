CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE IF NOT EXISTS users (
    id uuid DEFAULT uuid_generate_v4(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR (75) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    CONSTRAINT pk_users PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS short_link (
    id uuid DEFAULT uuid_generate_v4(),
    owner_id uuid NOT NULL,
    hash VARCHAR(255) UNIQUE NOT NULL,
    long_url VARCHAR(255) NOT NULL,
    is_private BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    CONSTRAINT pk_sl PRIMARY KEY (id),
    CONSTRAINT fk_sl_owner FOREIGN KEY (owner_id) REFERENCES users(id)
);