-- Add up migration script here
CREATE TABLE theaters
(
    id             UUID PRIMARY KEY,
    name           TEXT,
    ugc_identifier INT2
);

CREATE TABLE movies
(
    id       UUID PRIMARY KEY,
    title    TEXT UNIQUE NOT NULL,
    grade    REAL,
    synopsis TEXT
);

CREATE TABLE screenings
(
    id UUID PRIMARY KEY,
    movie_id UUID,
    theater_id UUID,
    screenings_time JSON,
    due_date DATE,
    FOREIGN KEY (theater_id) REFERENCES theaters(id) ON DELETE CASCADE,
    FOREIGN KEY (movie_id) REFERENCES movies(id) ON DELETE CASCADE
);

INSERT INTO theaters (id, name, ugc_identifier)
VALUES (gen_random_uuid(), 'confluence', 36),
       (gen_random_uuid(), 'astoria', 33),
       (gen_random_uuid(), 'part_dieu', 58),
       (gen_random_uuid(), 'cite_international', 32);