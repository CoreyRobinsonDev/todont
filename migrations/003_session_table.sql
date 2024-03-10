CREATE TABLE IF NOT EXISTS "session" (
    id UUID PRIMARY KEY NOT NULL,
    user_id UUID NOT NULL,
    FOREIGN KEY (user_id) REFERENCES t_user(id)
);
