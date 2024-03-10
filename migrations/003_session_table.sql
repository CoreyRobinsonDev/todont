DROP TABLE IF EXISTS "session";

CREATE TABLE IF NOT EXISTS "session" (
    id SERIAL PRIMARY KEY NOT NULL,
    user_id UUID NOT NULL,
    FOREIGN KEY (user_id) REFERENCES t_user(id)
);
