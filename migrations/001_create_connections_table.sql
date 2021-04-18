CREATE TABLE IF NOT EXISTS connections (
    name            TEXT    PRIMARY KEY NOT NULL,
    read_only       BOOLEAN NOT NULL,
    host            TEXT    NOT NULL,
    username        TEXT    NOT NULL,
    password        TEXT    NOT NULL,
    use_ssl         BOOLEAN NOT NULL,
    repl_set_name   TEXT,
    auth_source     TEXT
);
