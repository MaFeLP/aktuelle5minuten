import hashlib
from ..dlf import PREFIX as DLF_PREFIX
from flask import g, Flask
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


def _article_from_db_result(row: tuple):
    return {
        "key": row[0],
        "title": row[1],
        "teaserHeadline": row[2],
        "teaserText": row[3],
        "date": row[4],
        "localeDate": row[5],
        "href": row[6],
    }


def get_article_from_key(db: Connection, app, key):
    cursor = db.cursor()
    result = cursor.execute(GET_ARTICLE_KEY, key).fetchall()
    article = _article_from_db_result(result[0])
    app.logger.debug(article)
    return article


def get_categories(db: Connection) -> list:
    cursor = db.cursor()
    return [row[0] for row in cursor.execute(GET_CATEGORIES).fetchall()]


def get_first_article(db: Connection, app: Flask):
    cursor = db.cursor()
    result = cursor.execute(FIRST_ARTICLE).fetchall()
    article = _article_from_db_result(result[0])
    app.logger.debug(article)
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
