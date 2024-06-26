CREATE TABLE IF NOT EXISTS 'articles' (
    -- Data extracted from wochenuberblick
    key varchar(255) UNIQUE NOT NULL PRIMARY KEY,
    title text NOT NULL,
    teaserHeadline text NOT NULL,
    teaserText text NOT NULL,
    date timestamp NOT NULL,
    localeDate text NOT NULL,

    -- Data from the full article
    kicker text DEFAULT NULL,
    description text DEFAULT NULL,
    content TEXT DEFAULT NULL,
    category varchar(63) DEFAULT NULL,

    -- Internal status keeping:
    -- 0: Not categorized
    -- 1: Accepted, please print
    -- 2: Demoted, don't print
    -- 3: Bullet points already created; included in print_articles
    status integer NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS 'print_articles' (
    id integer PRIMARY KEY AUTOINCREMENT ,
    category varchar(63) NOT NULL,
    bullets text NOT NULL,
    created_at timestamp DEFAULT CURRENT_TIMESTAMP,
    printed boolean DEFAULT false
);
