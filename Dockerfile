FROM rust:1.85-slim-bullseye AS builder
RUN apt-get update && apt-get install -y sqlite3 libsqlite3-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/ugc-scrapper
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim AS base
RUN apt-get update && apt-get install -y sqlite3 libsqlite3-dev && rm -rf /var/lib/apt/lists/*

FROM base AS api
COPY --from=builder /usr/local/cargo/bin/api /usr/local/bin/api
CMD ["api"]