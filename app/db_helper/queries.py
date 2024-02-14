CLEAN_ARTICLES = """DELETE FROM `articles`"""

INSERT_ARTICLES = """
INSERT OR IGNORE INTO `articles`
    (key, title, teaserHeadline, teaserText, date, localeDate, href)
VALUES
    (  ?,     ?,              ?,          ?,    ?,          ?,    ?)
"""

FIRST_ARTICLE = """
SELECT *
FROM `articles`
WHERE `status` = 0
ORDER BY `date`
LIMIT 1 
"""

GET_ARTICLE_KEY = """SELECT * FROM `articles` WHERE `key` = (?)"""

GET_ARTICLES_CATEGORY = (
    """SELECT title, description, content FROM `articles` WHERE `category` = (?)"""
)

GET_CATEGORIES = """SELECT DISTINCT `category` FROM `articles` WHERE `status` = 1"""

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

GET_PRINT_ARTICLES = """SELECT category, bullets FROM `print_articles`"""
