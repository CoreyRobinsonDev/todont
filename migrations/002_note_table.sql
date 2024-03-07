DROP TABLE IF EXISTS "note";

CREATE TABLE IF NOT EXISTS "note" (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL,
    title VARCHAR(63) NOT NULL,
    description VARCHAR(255),
    completed BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    completed_at TIMESTAMP WITH TIME ZONE,
    FOREIGN KEY (user_id) REFERENCES user(id)
);
