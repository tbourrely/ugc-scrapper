CREATE TABLE theaters(
    id TEXT PRIMARY KEY,
    name TEXT,
    ugc_identifier INT
);

CREATE TABLE movies(
    id TEXT PRIMARY KEY,
    title TEXT,
    grade FLOAT,
    synopsis TEXT,
);

CREATE TABLE screenings (
    id TEXT PRIMARY KEY
    movie_id TEXT
    theater_id TEXT
    screenings_time JSON
    due_date timestamp
    FOREIGN KEY (theater_id) REFERENCES theaters(id) ON DELETE CASCADE
    FOREIGN KEY (movie_id) REFERENCES movies(id) ON DELETE CASCADE
)

INSERT INTO theaters (id, name, ugc_identifier) VALUES
    (NEWID(), 'confluence', 36),
    (NEWID(), 'astoria', 33),
    (NEWID(), 'part_dieu', 58),
    (NEWID(), 'cite_international', 32);