-- This file should undo anything in `up.sql`
ALTER TABLE articles DROP COLUMN content_html;

ALTER TABLE articles DROP COLUMN figure_src;
ALTER TABLE articles DROP COLUMN figure_alt;
ALTER TABLE articles DROP COLUMN figure_srcset;
ALTER TABLE articles DROP COLUMN figure_title;
ALTER TABLE articles DROP COLUMN figure_caption;
