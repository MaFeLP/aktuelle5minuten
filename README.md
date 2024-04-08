# Aktuelle 5 Minuten
Semi-automatische PDF-Erstellung mit den News der letzten Woche vom Deutschlandfunk

## Set-Up
```shell
mkdir -p data/pdfs
docker pull ghcr.io/MaFeLP/aktuelle5minuten:main
docker run -d --rm \
    --name aktuelle5minuten \
    -p 8080:80 \
    -v ./data:/data:rw \
    ghcr.io/MaFeLP/aktuelle5minuten:main
```

## Build
> [!NOTE]
> You need to change the server URL for your API to match your environment.
> To set up your environment for development, run the following command. For production,
> replace `http://localhost:5000` with your own domain name/IP.

```shell
sed -E -i 's|export const BASE_PATH.*$|export const BASE_PATH = "http://localhost:5000";|g' src/api-client/runtime.ts
docker build -t aktuelle5minuten .
```

## Development
> [!NOTE]
> Needs [Python Poetry](https://python-poetry.org/) and [pnpm](https://pnpm.io/) installed.

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
