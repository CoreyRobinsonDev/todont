DROP TABLE IF EXISTS t_user;

CREATE TABLE IF NOT EXISTS t_user (
    id UUID PRIMARY KEY,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    UNIQUE (id, email)
);
