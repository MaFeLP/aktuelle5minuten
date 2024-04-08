import datetime
from logging.config import dictConfig
import os
import tempfile
import typst
from flask import (
    Flask,
    abort,
    send_from_directory,
    send_file,
    g,
    Response,
    redirect,
    request,
)
from .db_helper import (
    get_db,
    insert_articles,
    update_article_contents,
    clean,
    count,
    get_article_from_key,
    get_first_article,
    get_categories,
    insert_bullets,
    promote_article as db_promote_article,
    demote_article as db_demote_article,
    get_articles_from_category,
    mark_category_printed,
    get_print_articles,
    mark_bullets_as_printed,
)
from .dlf import (
    download_article,
    download_wochenrueckblick,
    parse_article,
    parse_wochenrueckblick,
    PREFIX as DLF_PREFIX,
)
import re

_DEFAULT_CATEGORIES = [
    "Aktuelles Ereignis",
    "Außenpolitik",
    "Hamburg",
    "Politik",
    "Sonstiges",
    "USA",
    "Wirtschaft",
]

dictConfig({
    'version': 1,
    'formatters': {'default': {
        'format': '[%(asctime)s][%(module)s][%(levelname)s] %(message)s',
    }},
    'handlers': {'wsgi': {
        'class': 'logging.StreamHandler',
        'stream': 'ext://flask.logging.wsgi_errors_stream',
        'formatter': 'default'
    }},
    'root': {
        'level': 'INFO',
        'handlers': ['wsgi']
    }
})

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


@app.route("/api/article/get/first")
def first_article():
    article = get_first_article(get_db(), app)
    if article is None:
        return Response("No articles found", 404)
    try:
        html = download_article(DLF_PREFIX + article["key"])
        parsed = parse_article(html)
        update_article_contents(get_db(), parsed)
        return parsed
    except ConnectionError as ex:
        app.logger.warn('Could not download article %s; Error: %s', article["key"], ex)
        db_demote_article(get_db(), article["key"])
        return first_article()


@app.route("/api/article/get")
def article_get():
    key = request.args["key"]
    html = download_article(DLF_PREFIX + key)
    parsed = parse_article(html)
    update_article_contents(get_db(), parsed)
    return parsed


@app.route("/api/category/bullets", methods=["POST"])
def add_bullets() -> Response:
    def typst_escape(string: str) -> str:
        string = string.replace("#", r"\#")
        string = string.replace("\\", r"\\")
        string = string.replace("$", r"\$")
        return string

    if "category" not in request.form or "bullets" not in request.form:
        abort(406, "Category or Bullets missing")

    category, bullets = typst_escape(request.form["category"]), typst_escape(
        request.form["bullets"]
    )

    insert_bullets(get_db(), category, bullets)
    mark_category_printed(get_db(), category)

    # Handle: all categories converted to bullet points
    if len(get_categories(get_db())) == 0:
        today = datetime.datetime.now()
        with tempfile.NamedTemporaryFile() as tmp:
            tmp.write(
                b"""#set document(author: "Max Ove Fehlinger", title: "Aktuelle 5 Minuten")
#set page(numbering: "1", number-align: center)
#set par(justify: true)
#set text(font: "New Computer Modern", lang: "de")
#show math.equation: set text(weight: 400)
#show heading: set text(font: "New Computer Modern Sans")
#set heading(numbering: "1.1")
#align(center)[
  #block(text(font: "New Computer Modern Sans", weight: 700, 1.75em, "Aktuelle 5 Minuten"))
  #v(1em, weak: true)
  #datetime.today().display("[day].[month].[year]") #sym.dash.em #strong("Max Ove Fehlinger")
]

"""
            )

            for category, bullets in get_print_articles(get_db()):
                tmp.write(b"= " + str.encode(category) + b"\n")
                tmp.write(str.encode(bullets) + b"\n")
            tmp.flush()
            typst.compile(
                input=tmp.name,
                output=f"{app.root_path}/pdfs/{today.strftime('%Y-%m-%d_%H:%M:%S')}.pdf",
            )
            mark_bullets_as_printed(get_db())
            return redirect(f"/files/{today.strftime('%Y-%m-%d_%H:%M:%S')}.pdf")

    return redirect("/pdfcreate")


@app.route("/api/category/all")
def categories():
    db_categories = get_categories(get_db())

    if request.args["print"] == "true":
        return [] if db_categories is None else db_categories

    if db_categories is None:
        return sorted(_DEFAULT_CATEGORIES)
    for category in _DEFAULT_CATEGORIES:
        if category not in db_categories:
            db_categories.append(category)
    return sorted(db_categories)


@app.route("/api/ai")
def ai_status():
    chatgpt, claude = False, False
    if os.environ.get("ENABLE_CHATGPT", None) in ["1", "True", "TRUE", "true"]:
        chatgpt = True
    if os.environ.get("ENABLE_CLAUDE", None) in ["1", "True", "TRUE", "true"]:
        claude = True
    return {"chatgpt": chatgpt, "claude": claude}


@app.route("/api/category/summary")
def get_articles_by_category():
    category = request.args["category"]
    results = get_articles_from_category(get_db(), category)
    out = ""
    for result in results:
        out += f"""# {result[0]}
{result[1]}
{result[2]}

"""

        out = re.sub(
            r"Diese Nachricht wurde am \d{2}\.\d{2}\.\d{4} im Programm Deutschlandfunk gesendet\.",
            "",
            out,
        )

    return {
        "category": category,
        "text": out.strip(),
    }


@app.route("/api/actions/clean")
def clean_articles():
    clean(get_db())
    return Response("Database cleared", 200)


@app.route("/api/count")
def count_articles():
    return count(get_db())


@app.route("/api/files")
def list_files():
    os.makedirs(app.root_path + "/pdfs", exist_ok=True)
    files = os.listdir(app.root_path + "/pdfs")
    return sorted(files, reverse=True)


@app.route("/files/<path:path>")
def download_pdfs(path):
    os.makedirs(app.root_path + "/pdfs", exist_ok=True)
    return send_from_directory("pdfs", path)


@app.route("/api/actions/load")
def load_articles_to_db():
    wr = download_wochenrueckblick()
    parsed = parse_wochenrueckblick(wr)
    insert_articles(get_db(), parsed)
    return Response("Created", 201)


@app.route("/api/article/demote")
def demote_article():
    db_demote_article(get_db(), request.args["key"])
    return Response("Ok", 201)


@app.route("/api/article/promote")
def promote_article():
    category, key = request.args["category"], request.args["key"]
    if len(category) > 63:
        return Response("Category too long. Maximum of 63 characters allowed", 400)
    db_promote_article(get_db(), key, category)
    return Response("Ok", 201)


if __name__ == "__main__":
    app.run(host="0.0.0.0")
