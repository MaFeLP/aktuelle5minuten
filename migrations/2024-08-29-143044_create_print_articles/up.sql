CREATE TABLE IF NOT EXISTS 'print_articles' (
    id integer PRIMARY KEY AUTOINCREMENT ,
    category varchar(63) NOT NULL,
    bullets text NOT NULL,
    created_at timestamp DEFAULT CURRENT_TIMESTAMP,
    printed boolean DEFAULT false
);
