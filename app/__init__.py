import datetime
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
    get_article_from_key,
    get_first_article,
    get_categories,
    insert_bullets,
    promote_article as db_promote_article,
    demote_article as db_demote_article,
    get_articles_from_category,
    mark_category_printed,
    get_print_articles,
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
    parsed = parse_article(html)
    update_article_contents(get_db(), parsed)
    return parsed


@app.route("/article/<string:ref>")
def article_get(ref: str):
    html = download_article(DLF_PREFIX + ref)
    parsed = parse_article(html)
    update_article_contents(get_db(), parsed)
    return parsed


@app.route("/add/<string:ref>")
def add_article(ref: str):
    html = download_article(DLF_PREFIX + ref)
    article = parse_article(html)
    update_article_contents(get_db(), article)
    return Response("Created", 201)


@app.route("/bullets", methods=["POST"])
def add_bullets() -> Response:
    def typst_escape(string: str) -> str:
        string = string.replace("#", "\\#")
        string = string.replace("\\", "\\\\")
        string = string.replace("$", "\\$")
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
            return redirect(f"/files/{today.strftime('%Y-%m-%d_%H:%M:%S')}.pdf")

    return redirect("/pdfcreate")


@app.route("/categories")
def categories():
    db_categories = get_categories(get_db())
    if db_categories is None:
        return sorted(_DEFAULT_CATEGORIES)
    for category in _DEFAULT_CATEGORIES:
        if category not in db_categories:
            db_categories.append(category)
    return sorted(db_categories)


@app.route("/chatgpt")
def chatgpt():
    enabled = False
    env = os.environ.get("ENABLE_CHATGPT", None)
    if os.environ.get("ENABLE_CHATGPT", None) in ["1", "True", "TRUE", "true"]:
        enabled = True
    return {"enabled": enabled}


@app.route("/print_categories")
def print_categories():
    return sorted(get_categories(get_db()))


@app.route("/category/<string:category>")
def get_articles_by_category(category: str):
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
