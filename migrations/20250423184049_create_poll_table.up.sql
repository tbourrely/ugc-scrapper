-- Add up migration script here
CREATE TABLE polls
(
    id             uuid PRIMARY KEY,
    distant_id     uuid,
    type           smallint,
    created_at     timestamp default current_timestamp
);

CREATE TABLE movies_seen
(
    id          uuid PRIMARY KEY,
    content     varchar,
    created_at  timestamp
)