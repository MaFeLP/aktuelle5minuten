CLEAN_ARTICLES = """DELETE FROM `articles`"""
CLEAN_PRINT_ARTICLES = """DELETE FROM `print_articles`"""

INSERT_ARTICLES = """
INSERT OR IGNORE INTO `articles`
           (key, title, teaserHeadline, teaserText, date, localeDate, href)
    VALUES (  ?,     ?,              ?,          ?,    ?,          ?,    ?);
"""

FIRST_ARTICLE = """
SELECT * FROM `articles` SORT ORDER BY `date` DESC LIMIT 1
"""

GET_ARTICLE_KEY = """SELECT * FROM `articles` WHERE `key` = (?)"""

GET_CATEGORIES = """SELECT DISTINCT `category` FROM `print_articles`"""

INSERT_PRINT_ARTICLES = """
INSERT OR IGNORE INTO `print_articles`
           (hash, kicker, title, description, content, date, category)
    VALUES (   ?,     ?,     ?,           ?,        ?,    ?,        ?);
"""

REMOVE = """DELETE FROM `articles` WHERE `key` = (?)"""
