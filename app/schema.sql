CREATE TABLE IF NOT EXISTS 'articles' (
    -- Data extracted from wochenuberblick
    key varchar(63) UNIQUE NOT NULL PRIMARY KEY,
    title text NOT NULL,
    teaserHeadline text NOT NULL,
    teaserText text NOT NULL,
    date timestamp NOT NULL,
    localeDate text NOT NULL,
    href text NOT NULL,

    -- Data from the full article
    kicker text DEFAULT NULL,
    description text DEFAULT NULL,
    content TEXT DEFAULT NULL,
    category varchar(63) DEFAULT NULL,

    -- Internal status keeping:
    -- 0: Not categorized
    -- 1: Accepted, please print
    -- 2: Demoted, don't print
    status integer NOT NULL DEFAULT 0
);

--CREATE TABLE IF NOT EXISTS 'print_articles' (
--    hash varchar(40) NOT NULL PRIMARY KEY, -- represents a SHA1 of: kicker+title+description+content
--);
