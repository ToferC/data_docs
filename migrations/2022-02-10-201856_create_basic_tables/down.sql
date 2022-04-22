-- This file should undo anything in `up.sql`

DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS email_verification_code;
DROP TABLE IF EXISTS password_reset_token;

DROP TABLE IF EXISTS subjects;
DROP TABLE IF EXISTS categories;
DROP TABLE IF EXISTS keywords;

DROP TABLE IF EXISTS metadata;
DROP TABLE IF EXISTS texts;
DROP TABLE IF EXISTS sections;
DROP TABLE IF EXISTS documents;
DROP TABLE IF EXISTS template_sections;
DROP TABLE IF EXISTS templates;

