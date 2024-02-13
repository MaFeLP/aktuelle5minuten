import hashlib
from ..dlf import PREFIX as DLF_PREFIX
from flask import g, Flask, abort
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
    db.commit()


def demote_article(db: Connection, key: str):
    cursor = db.cursor()
    cursor.execute(DEMOTE_ARTICLE, [key,])
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
        "kicker": row[7],
        "description": row[8],
        "content": row[9],
        "category": row[10],
        "status": row[11],
    }


def get_article_from_key(db: Connection, app, key: str) -> dict | None:
    cursor = db.cursor()
    result = cursor.execute(GET_ARTICLE_KEY, [key]).fetchall()
    if len(result) == 0:
        return None
    article = _article_from_db_result(result[0])
    app.logger.debug(article)
    return article


def get_categories(db: Connection) -> list:
    cursor = db.cursor()
    return [row[0] for row in cursor.execute(GET_CATEGORIES).fetchall() if row[0] is not None]


def get_first_article(db: Connection, app: Flask) -> dict | None:
    cursor = db.cursor()
    result = cursor.execute(FIRST_ARTICLE).fetchall()
    if len(result) == 0:
        return None
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


def update_article_contents(db: Connection, article: dict[str, str]):
    db.cursor().execute(INSERT_ARTICLE_CONTENT, (
        article["kicker"],
        article["description"],
        article["content"]["plaintext"],
        article["key"]
    ))
    db.commit()


def promote_article(db: Connection, key: str, category: str):
    if len(category) > 63:
        abort(400)
    db.cursor().execute(PROMOTE_ARTICLE, (key, category))
    db.commit()
