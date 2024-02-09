CREATE TABLE IF NOT EXISTS 'articles' (
    key varchar(63) UNIQUE NOT NULL PRIMARY KEY,
    title text NOT NULL DEFAULT '',
    teaserHeadline text NOT NULL DEFAULT '',
    teaserText text NOT NULL DEFAULT '',
    date TIMESTAMP NOT NULL,
    localeDate text NOT NULL DEFAULT '',
    href text NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS 'print_articles' (
    hash varchar(40) NOT NULL PRIMARY KEY, -- represents a SHA1 of: kicker+title+description+content
    kicker text NOT NULL DEFAULT '',
    title text NOT NULL DEFAULT '',
    description text NOT NULL DEFAULT '',
    content TEXT NOT NULL DEFAULT ''
);
