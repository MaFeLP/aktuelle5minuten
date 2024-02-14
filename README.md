# Aktuelle 5 Minuten
Semi-automatische PDF-Erstellung mit den News der letzten Woche vom Deutschlandfunk

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
