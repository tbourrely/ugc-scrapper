-- Add up migration script here
CREATE TABLE polls
(
    id             uuid PRIMARY KEY,
    distant_id     uuid,
    type           smallint,
    created_at     timestamp default current_timestamp
);

CREATE TABLE answer
(
    id          uuid PRIMARY KEY,
    poll_id     uuid,
    content     varchar,
    created_at  timestamp,
    FOREIGN KEY (poll_id) REFERENCES polls(id) ON DELETE CASCADE
)