from flask import Flask
from .dlf import (
    download_article,
    download_wochenrueckblick,
    parse_article,
    parse_wochenrueckblick,
)

app = Flask(__name__)


@app.route("/")
def hello_world():
    return "Hello World!"


@app.route("/articles")
def articles_get():
    wr = download_wochenrueckblick()
    return parse_wochenrueckblick(wr)


@app.route("/article/<string:ref>")
def article_get(ref: str):
    html = download_article(f"https://www.deutschlandfunk.de/{ref}")
    return parse_article(html)


if __name__ == "__main__":
    app.run()
