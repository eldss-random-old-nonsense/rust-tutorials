-- Your SQL goes here
CREATE TABLE task (
    id INTEGER NOT NULL,
    title TEXT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT 0,
    PRIMARY KEY (id)
);