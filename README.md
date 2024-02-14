# Aktuelle 5 Minuten
Semi-automatische PDF-Erstellung mit den News der letzten Woche vom Deutschlandfunk

## Set-Up
```shell
mkdir -p data/pdfs
docker pull ghcr.io/MaFeLP/aktuelle5minuten:latest
docker run -d --rm \
    -p 8080:80 \
    -v ./data:/data:rw \
    ghcr.io/MaFeLP/aktuelle5minuten:latest
```

## Build
```shell
docker build -t aktuelle5minuten .
```

## Development
> **Note**: Needs [Python Poetry](https://python-poetry.org/) and [pnpm](https://pnpm.io/) installed.

### Start the backend service
```shell
poetry install
poetry run flask run
```

### Start the frontend dev server
```shell
pnpm install
pnpm run dev
```
