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

## Configuration
Aktuelle 5 Minuten is configured via environment variables. Use the `-e A5M_CONFIG=foobar` command line option to run the docker container.
You can also an `.env` file with the `--env-file aktuelle5minuten.env` command file option.

| Environment Variable | Description | Value | Default |
| --- | --- | --- | --- |
| `A5M_AI_CHATGPT` | If the ChatGPT button should be displayed on the bullet point creation page. | boolean | `false` |
| `A5M_AI_CLAUDE` | If the Claude AI button should be displayed on the bullet point creation page. | boolean | `false` |
| `A5M_ASSETS_PATH` | Where the html and javascript files are stored **only change when you know what you are doing** | Path (string) | `/usr/local/share/aktuelle5minuten/` |
| `A5M_DATA_PATH` | Where the data (created pdfs, the sqlite-database) should be stored. | Path (string) | `/data/` |
| `A5M_FONTS_DIR` | Where the fonts for the pdf creation should be stored | Path (string) | `/usr/share/fonts/liberation/` |
| `A5M_PDF_AUTHOR`| The author that should be used in the generated PDF  | String | `Default Author` |
| `A5M_PDF_TITLE` | The title of the PDF | String | `Aktuelle 5 Minuten` |
| `TZ` | The timezone used to determine local time. Used for example in the pdf date determination | String | `Etc/UTC` |


## Build
Building the service is as easy as can be: Install rust and it's tooling, and then run the following
command:

```bash
cargo run
```

You can also build a production version using:

```bash
cargo build --release
```

There is also a docker image available that bundles everything into a small package:

```bash
docker build -t aktuelle5minuten .
```

