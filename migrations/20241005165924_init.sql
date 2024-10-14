-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE OR REPLACE FUNCTION manage_updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE users (
    id uuid NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
    name text NOT NULL,
    email text NOT NULL,
    avatar_url text,
    updated_at timestamptz NOT NULL DEFAULT now(),
    created_at timestamptz NOT NULL DEFAULT now()
);

SELECT manage_updated_at('users');
CREATE INDEX idx_users_login ON users (email);

INSERT INTO users (id, name, email, avatar_url)
VALUES (
    '00000000-0000-0000-0000-000000000000',
    'Developer user',
    'developer.user@gmail.com',
    'https://avatars.githubusercontent.com/u/116913262?v=4'
);
