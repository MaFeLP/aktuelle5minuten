# Create the rust backend
FROM rust:1.89-slim-bookworm AS backend
RUN apt-get update && apt-get install -y \
    libsqlite3-dev libssl-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /build
COPY . ./
#RUN --mount=type=cache,target=/build/target \
#    --mount=type=cache,target=/usr/local/cargo/registry \
#    --mount=type=cache,target=/usr/local/cargo/git \
#    set -eux && \
#    cargo build --release && \
#    objcopy --compress-debug-sections target/release/aktuelle5minuten ./main
RUN cargo build --release && objcopy --compress-debug-sections target/release/aktuelle5minuten ./main

FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y \
    fonts-liberation \
    sqlite3 \
    openssl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY ./assets /app/assets/
COPY ./templates /app/templates/
COPY --from=backend /build/main /app/aktuelle5minuten

# Default environment variables. Should only be touched, if you know what you are doing.
ENV ROCKET_DATABASES='{sqlite_db={url="/data/db.sqlite3"}}'
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=80
# The path where static assets should be stored. Default assets are available in /app/assets/
ENV A5M_ASSETS_PATH='/app/assets/'

### User configuration, but provided default variables
# The path where all the data should be stored
ENV A5M_DATA_PATH='/data'
# Where typst should look for additional fonts. Currently only support one directory (with
# subdirectories)
ENV A5M_FONTS_DIR='/usr/share/fonts/truetype/liberation/'
# The name of the PDF Author
ENV A5M_PDF_AUTHOR='Default Author'
# The default pdf title
ENV A5M_PDF_TITLE='Aktuelle 5 Minuten'
### Enables the ChatGPT and Claude buttons
ENV A5M_AI_CHATGPT=1
ENV A5M_AI_CLAUDE=1

# Deployment configuration
EXPOSE 80/tcp
VOLUME "/data"

CMD ["/app/aktuelle5minuten"]
