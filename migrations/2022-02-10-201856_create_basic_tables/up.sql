-- Your SQL goes here

CREATE TABLE users (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    hash BYTEA NOT NULL,
    salt VARCHAR(255) NOT NULL,
    email VARCHAR(128) NOT NULL UNIQUE,
    user_name VARCHAR(32) NOT NULL UNIQUE,
    slug VARCHAR(32) NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    role VARCHAR(32) NOT NULL DEFAULT 'user',
    validated bool NOT NULL DEFAULT false
);

CREATE UNIQUE INDEX users__email_idx ON users(email);

CREATE TABLE IF NOT EXISTS email_verification_code (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    email_address VARCHAR(128) UNIQUE NOT NULL,
    activation_code VARCHAR(5) UNIQUE NOT NULL,
    expires_on TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS password_reset_token (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    email_address VARCHAR(128) UNIQUE NOT NULL,
    reset_token VARCHAR(36) UNIQUE NOT NULL,
    expires_on TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS templates (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    name_text_id UUID NOT NULL,
    purpose_text_id UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    slug VARCHAR(64) NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS template_sections (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    template_id UUID NOT NULL,
    FOREIGN KEY(template_id)
        REFERENCES templates(id) ON DELETE CASCADE,
    header_text_id UUID NOT NULL,
    order_number INT NOT NULL,
    instructions_text_id UUID NOT NULL,
    help_text_id UUID NOT NULL,
    character_limit INT 
);

CREATE TABLE IF NOT EXISTS documents (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    purpose_text_id UUID NOT NULL,
    publishable bool NOT NULL DEFAULT false
);

CREATE TABLE IF NOT EXISTS sections (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    document_id UUID NOT NULL,
    FOREIGN KEY(document_id)
        REFERENCES documents(id) ON DELETE CASCADE,
    template_section_id UUID NOT NULL,
    FOREIGN KEY(template_section_id)
        REFERENCES template_sections(id)
);

CREATE TABLE IF NOT EXISTS texts (
    id UUID DEFAULT gen_random_uuid() NOT NULL,
    section_id UUID,
    FOREIGN KEY(section_id)
        REFERENCES sections(id) ON DELETE CASCADE,
    lang VARCHAR(2) NOT NULL default 'en',
    content VARCHAR NOT NULL,
    translated bool NOT NULL DEFAULT false,
    machine_translation bool NOT NULL default false,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY(id, lang)
);