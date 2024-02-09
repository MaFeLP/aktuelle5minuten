CLEAN_ARTICLES = """DELETE FROM `articles`"""
CLEAN_PRINT_ARTICLES = """DELETE FROM `print_articles`"""

INSERT_ARTICLES = """
INSERT OR IGNORE INTO `articles`
           (key, title, teaserHeadline, teaserText, date, localeDate, href)
    VALUES (  ?,     ?,              ?,          ?,    ?,          ?,    ?);
"""

GET_ARTICLE_KEY = """SELECT * FROM `articles` WHERE `key` = (?)"""

INSERT_PRINT_ARTICLES = """
INSERT OR IGNORE INTO `print_articles`
           (hash, kicker, title, description, content)
    VALUES (   ?,     ?,     ?,           ?,       ?);
"""

REMOVE = """DELETE FROM `articles` WHERE `key` = (?)"""
