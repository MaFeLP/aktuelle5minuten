-- Your SQL goes here
ALTER TABLE `articles` RENAME COLUMN 'teaserHeadline' TO 'teaser_headline';
ALTER TABLE `articles` RENAME COLUMN 'teaserText' TO 'teaser_text';
ALTER TABLE `articles` RENAME COLUMN 'localeDate' TO 'locale_date';
