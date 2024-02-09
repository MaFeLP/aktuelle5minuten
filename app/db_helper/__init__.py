import hashlib
from ..dlf import PREFIX as DLF_PREFIX
from flask import g
from sqlite3 import Connection, connect
from .queries import *

_DATABASE = "./db.sqlite3"


def _sha1sum(data):
    sha1 = hashlib.sha1()
    sha1.update(data.encode("utf-8"))
    return sha1.hexdigest()


def get_db():
    db = getattr(g, "_database", None)
    if db is None:
        db = g._database = connect(_DATABASE)
    return db


def clean(db: Connection):
    cursor = db.cursor()
    cursor.execute(CLEAN_ARTICLES)
    cursor.execute(CLEAN_PRINT_ARTICLES)
    db.commit()


def delete_article(db: Connection, key):
    cursor = db.cursor()
    cursor.execute(REMOVE, key)
    db.commit()


def get_article_from_key(db: Connection, key):
    cursor = db.cursor()
    article = cursor.execute(GET_ARTICLE_KEY, key)
    print(article)
    return article


def insert_articles(db: Connection, articles: list[dict]):
    cursor = db.cursor()
    non_articles = []
    for article in articles:
        href = article["href"]
        if not href.startswith(DLF_PREFIX):
            non_articles.append(href)
        href = href[len(DLF_PREFIX) :]
        key = article["key"]
        title = article["title"]
        teaser_headline = article["teaserHeadline"]
        teaser_text = article["teaserText"]
        date = article["date"]
        locale_date = article["localeDate"]

        elements = (key, title, teaser_headline, teaser_text, date, locale_date, href)

        cursor.execute(INSERT_ARTICLES, elements)
    db.commit()


def insert_print_article(db: Connection, article: dict[str, str]):
    # Get required elements
    kicker = article["kicker"]
    title = article["title"]
    description = article["description"]
    content = article["content"]["plaintext"]
    elements = (kicker, title, description, content)
    element_hash = _sha1sum("+".join(elements))

    db.cursor().execute(INSERT_PRINT_ARTICLES, (element_hash, *elements))
    db.commit()
