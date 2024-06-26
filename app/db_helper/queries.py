CLEAN_ARTICLES = """DELETE FROM `articles` WHERE date < datetime('now', '-1 month')"""
CLEAN_PRINT_ARTICLES = (
    """DELETE FROM `print_articles` WHERE created_at < datetime('now', '-1 month')"""
)

COUNT_ARTICLES = """SELECT COUNT(*) FROM articles WHERE status = 0"""
COUNT_ARTICLES_BY_DATE = (
    """SELECT COUNT(*) FROM articles WHERE status = 0 AND DATE(date) = (?)"""
)
COUNT_CATEGORIES = """SELECT COUNT(DISTINCT category) FROM articles WHERE status = 1"""

INSERT_ARTICLES = """
INSERT OR IGNORE INTO `articles`
    (key, title, teaserHeadline, teaserText, date, localeDate)
VALUES
    (  ?,     ?,              ?,          ?,    ?,          ?)
"""

FIRST_ARTICLE = """
SELECT *
FROM `articles`
WHERE `status` = 0
ORDER BY `date`
LIMIT 1 
"""

FIRST_ARTICLE_BY_DATE = """
SELECT *
FROM `articles`
WHERE `status` = 0 AND DATE(date) = (?)
ORDER BY `date`
LIMIT 1 
"""

GET_ARTICLE_KEY = """SELECT * FROM `articles` WHERE `key` = (?)"""

GET_ARTICLES_CATEGORY = (
    """SELECT title, description, content FROM `articles` WHERE `category` = (?)"""
)

GET_CATEGORIES = """SELECT DISTINCT `category` FROM `articles` WHERE `status` = 1"""

GET_ARTICLE_DATES = (
    """SELECT DISTINCT DATE(`date`) FROM `articles` WHERE `status` = 0 ORDER BY date"""
)

INSERT_ARTICLE_CONTENT = """
UPDATE `articles`
SET `kicker` = (?),
    `description` = (?),
    `content` = (?)
WHERE
    `key` = (?)
"""

PROMOTE_ARTICLE = """
UPDATE `articles`
SET `status` = 1,
    `category` = (?)
WHERE
    `key` = (?)
"""

DEMOTE_ARTICLE = """
UPDATE `articles` 
SET `status` = 2
WHERE
    `key` = (?)
"""

MARK_CATEGORY_PRINTED = """
UPDATE `articles` 
SET `status` = 3
WHERE
    `category` = (?)
"""

INSERT_BULLETS = """
INSERT INTO `print_articles`
    (category, bullets)
VALUES
    (       ?,       ?)
"""

GET_PRINT_ARTICLES = (
    """SELECT category, bullets FROM `print_articles` WHERE printed = false"""
)

MARK_BULLETS_PRINTED = """UPDATE `print_articles` SET printed = true"""
