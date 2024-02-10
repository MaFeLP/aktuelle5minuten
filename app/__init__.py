import os
from flask import Flask, send_from_directory, send_file, g, Response
from .db_helper import (
    get_db,
    insert_articles,
    update_article_contents,
    clean,
    get_article_from_key,
    get_first_article, get_categories,
    promote_article as db_promote_article,
    demote_article as db_demote_article,
)
from .dlf import (
    download_article,
    download_wochenrueckblick,
    parse_article,
    parse_wochenrueckblick,
    PREFIX as DLF_PREFIX,
)

_DEFAULT_CATEGORIES = [
    "Aktuelles Ereignis",
    "Außenpolitik",
    "Hamburg",
    "Politik",
    "Sonstiges",
    "USA",
    "Wirtschaft",
]

app = Flask(__name__, static_url_path="", static_folder="static")


@app.teardown_appcontext
def close_connection(_exception):
    db = getattr(g, "_database", None)
    if db is not None:
        db.close()


@app.route("/")
@app.route("/tinder")
@app.route("/tinder/")
@app.route("/tinder/index.html")
@app.route("/tinder.html")
@app.route("/pdflist")
@app.route("/pdflist/")
@app.route("/pdflist/index.html")
@app.route("/pdflist.html")
@app.route("/pdfcreate")
@app.route("/pdfcreate/")
@app.route("/pdfcreate/index.html")
@app.route("/pdfcreate.html")
@app.route("/index.html")
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


@app.route("/article")
def first_article():
    article = get_first_article(get_db(), app)
    if article is None:
        return Response("No articles found", 404)
    html = download_article(DLF_PREFIX + article["href"])
    return parse_article(html)


@app.route("/article/<string:ref>")
def article_get(ref: str):
    html = download_article(DLF_PREFIX + ref)
    return parse_article(html)


@app.route("/add/<string:ref>")
def add_article(ref: str):
    html = download_article(DLF_PREFIX + ref)
    article = parse_article(html)
    update_article_contents(get_db(), article)
    return Response("Created", 201)


@app.route("/categories")
def categories():
    db_categories = get_categories(get_db())
    for category in _DEFAULT_CATEGORIES:
        if category not in db_categories:
            db_categories.append(category)
    return sorted(db_categories)


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


@app.route("/demote/<string:key>")
def demote_article(key: str):
    db_demote_article(get_db(), key)
    return Response("Ok", 201)


@app.route("/promote/<string:category>/<string:key>")
def promote_article(category: str, key: str):
    if len(category) > 63:
        return Response("Category too long. Maximum of 63 characters allowed", 400)
    db_promote_article(get_db(), category, key)
    return Response("Ok", 201)


if __name__ == "__main__":
    app.run(host="0.0.0.0")
