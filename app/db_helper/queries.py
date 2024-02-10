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

GET_CATEGORIES = """SELECT DISTINCT `category` FROM `articles`"""

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
