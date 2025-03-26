CREATE TABLE theaters(
    id TEXT PRIMARY KEY,
    name TEXT,
    ugc_identifier INT
);

CREATE TABLE movies(
    id INTEGER PRIMARY KEY,
    title TEXT,
    grade FLOAT,
    synopsis TEXT,
    screenings JSON,
    FOREIGN KEY (theater_id) REFERENCES theaters(id) ON DELETE CASCADE
);

INSERT INTO theaters (id, name, ugc_identifier) VALUES
    (NEWID(), 'confluence', 36),
    (NEWID(), 'astoria', 33),
    (NEWID(), 'part_dieu', 58),
    (NEWID(), 'cite_international', 32);