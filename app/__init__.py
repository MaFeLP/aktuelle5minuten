import os
from flask import Flask, send_from_directory, send_file, g, Response, request
from app.db_helper import get_db, insert_articles, insert_print_article, clean, get_article_from_key
from .dlf import (
    download_article,
    download_wochenrueckblick,
    parse_article,
    parse_wochenrueckblick,
    PREFIX as DLF_PREFIX,
)

app = Flask(__name__, static_url_path="", static_folder="static")


@app.teardown_appcontext
def close_connection(_exception):
    db = getattr(g, "_database", None)
    if db is not None:
        db.close()


@app.route("/")
def index():
    db = get_db()
    with app.open_resource("schema.sql", mode="r") as f:
        db.cursor().executescript(f.read())
    db.commit()
    return send_file("static/index.html")


@app.route("/<path:path>")
def serve(path):
    send_from_directory("static", path)


@app.route("/articles")
def articles_get():
    wr = download_wochenrueckblick()
    return parse_wochenrueckblick(wr)


@app.route("/article/<string:ref>", methods=['GET', 'DELETE'])
def article_get(ref: str):
    html = download_article(DLF_PREFIX + ref)
    return parse_article(html)


@app.route("/add/<string:ref>")
def add_article(ref: str):
    html = download_article(DLF_PREFIX + ref)
    article = parse_article(html)
    insert_print_article(get_db(), article)
    return Response("Created", 201)


@app.route("/clean")
def clean_articles():
    clean(get_db())
    return Response("Database cleared", 200)


@app.route("/files")
def list_files():
    os.makedirs(app.root_path + "/pdfs", exist_ok=True)
    files = os.listdir(app.root_path + "/pdfs")
    return sorted(files, reverse=True)


@app.route("/files/<path:path>")
def download_pds(path):
    os.makedirs(app.root_path + "/pdfs", exist_ok=True)
    return send_from_directory("pdfs", path)


@app.route("/load")
def load_articles_to_db():
    wr = download_wochenrueckblick()
    parsed = parse_wochenrueckblick(wr)
    insert_articles(get_db(), parsed)
    return Response("Created", 201)


@app.route("/promote/<string:tag>/<string:key>")
def promote_article(tag, key: str):
    article = get_article_from_key(get_db(), key)
    if article is None:
        return Response("Not found", 404)
    return Response("Ok", 200)


if __name__ == "__main__":
    app.run(host="0.0.0.0")
