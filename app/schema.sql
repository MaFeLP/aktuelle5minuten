CREATE TABLE IF NOT EXISTS 'articles' (
    key varchar(63) UNIQUE NOT NULL PRIMARY KEY,
    title text NOT NULL,
    teaserHeadline text NOT NULL,
    teaserText text NOT NULL,
    date timestamp NOT NULL,
    localeDate text NOT NULL,
    href text NOT NULL
);

CREATE TABLE IF NOT EXISTS 'print_articles' (
    hash varchar(40) NOT NULL PRIMARY KEY, -- represents a SHA1 of: kicker+title+description+content
    kicker text NOT NULL,
    title text NOT NULL,
    description text NOT NULL,
    content TEXT NOT NULL,
    date TIMESTAMP NOT NULL,
    category varchar(63) NOT NULL
);
