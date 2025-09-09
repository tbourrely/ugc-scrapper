FROM rust:1.85-slim-bullseye AS builder
RUN apt-get update && apt-get install -y sqlite3 libsqlite3-dev libssl-dev pkg-config ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/ugc-scrapper
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim AS base
RUN apt-get update && apt-get install -y sqlite3 libsqlite3-dev libssl-dev pkg-config ca-certificates cron && rm -rf /var/lib/apt/lists/*

FROM base
COPY --from=builder /usr/local/cargo/bin/scrapper /usr/local/bin/scrapper
COPY --from=builder /usr/local/cargo/bin/select_day /usr/local/bin/select_day
COPY --from=builder /usr/local/cargo/bin/select_movie /usr/local/bin/select_movie
ADD crontab .
RUN crontab -u root ./crontab
CMD [ "cron", "-f" ]
