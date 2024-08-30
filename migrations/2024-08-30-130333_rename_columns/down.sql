-- This file should undo anything in `up.sql`
ALTER TABLE `articles` RENAME COLUMN 'teaser_headline' TO 'teaserHeadline';
ALTER TABLE `articles` RENAME COLUMN 'teaser_text' TO 'teaserText';
ALTER TABLE `articles` RENAME COLUMN 'locale_date' TO 'localeDate';
