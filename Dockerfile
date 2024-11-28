# Build the svelte frontend with pnpm
FROM node:23-slim AS frontend
WORKDIR /build
RUN npm install -g pnpm
COPY frontend/package.json frontend/pnpm-lock.yaml ./
RUN pnpm install
COPY frontend/ ./
RUN pnpm build

# Create the rust backend
FROM rust:1.82-slim-bookworm AS backend
RUN apt-get update && apt-get install -y \
    libsqlite3-dev libssl-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /build
COPY --from=frontend /build/dist/index.html ./src/index.html
COPY . ./
RUN --mount=type=cache,target=/build/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    set -eux; \
    cargo build --release; \
    objcopy --compress-debug-sections target/release/aktuelle5minuten ./main


FROM debian:bookworm-slim
WORKDIR /data

RUN apt-get update && apt-get install -y \
    fonts-liberation \
    sqlite3 \
    openssl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=backend /build/main /usr/local/bin/aktuelle5minuten
COPY --from=frontend /build/dist /usr/local/share/aktuelle5minuten

# Default environment variables. Should only be touched, if you know what you are doing.
ENV ROCKET_DATABASES='{sqlite_db={url="/data/db.sqlite3"}}'
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=80

# Deployment configuration
EXPOSE 80/tcp
VOLUME "/data"

CMD ["aktuelle5minuten"]
