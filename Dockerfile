# Build the svelte frontend with pnpm
FROM node:21-slim AS frontend
WORKDIR /build
RUN npm install -g pnpm
COPY package.json pnpm-lock.yaml ./
RUN pnpm install
COPY . .
RUN pnpm build

# Package the python backend as a .whl
FROM python:3.12-bookworm AS flask
WORKDIR /build
RUN pip install poetry
# Copy the version files for better caching (also copy the README.md so peoptry doesn't complain)
COPY pyproject.toml poetry.lock README.md ./
RUN pip install poetry && poetry install --no-root --no-directory
# Copy the source files (copying from frontend, to also include the static frontend files)
COPY --from=frontend /build/app/ /build/app/
# Package the application
RUN poetry build -f wheel


FROM python:3.12-bookworm
WORKDIR /data
# sqlite is needed for the application to work
RUN apt-get update && apt-get install -y \
    sqlite3 \
    && rm -rf /var/lib/apt/lists/*
# The packaged sources
COPY --from=flask /build/dist/*.whl /
# Install the application and fix pdf-serving with a symlink:
# /usr/local/lib/python3.12/site-packages/app/pdfs -> /data/pdfs
RUN pip install --no-cache-dir /aktuelle5minuten-*-py3-none-any.whl \
    && rm -rf /usr/local/lib/python3.12/site-packages/app/pdfs \
    && mkdir /data/pdfs \
    && ln -s /data/pdfs/ /usr/local/lib/python3.12/site-packages/app/pdfs


EXPOSE 80/tcp
VOLUME "/data"

CMD ["gunicorn", "--bind", "0.0.0.0:80", "app:app"]

