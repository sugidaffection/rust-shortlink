CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR NOT NULL UNIQUE,
    email VARCHAR NOT NULL UNIQUE,
    password_hash VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE short_links (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID,
    anonymous_owner_id UUID,
    serial_id BIGINT GENERATED ALWAYS AS IDENTITY UNIQUE,
    hash VARCHAR UNIQUE,
    long_url TEXT NOT NULL,
    is_private BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ,
    click_count BIGINT NOT NULL DEFAULT 0,
    title VARCHAR,
    description TEXT,
    status VARCHAR NOT NULL CHECK (status IN ('active', 'inactive', 'expired')),

    CONSTRAINT fk_owner FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE link_clicks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    link_id UUID NOT NULL,
    clicked_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    referrer TEXT,
    user_agent TEXT,
    ip_address VARCHAR,
    country_code VARCHAR,

    CONSTRAINT fk_link FOREIGN KEY (link_id) REFERENCES short_links(id) ON DELETE CASCADE
);
