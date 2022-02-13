table! {
    documents (id) {
        id -> Uuid,
        purpose_text_id -> Uuid,
        publishable -> Bool,
    }
}

table! {
    email_verification_code (id) {
        id -> Uuid,
        email_address -> Varchar,
        activation_code -> Varchar,
        expires_on -> Timestamp,
    }
}

table! {
    password_reset_token (id) {
        id -> Uuid,
        email_address -> Varchar,
        reset_token -> Varchar,
        expires_on -> Timestamp,
    }
}

table! {
    sections (id) {
        id -> Uuid,
        document_id -> Uuid,
        template_section_id -> Uuid,
    }
}

table! {
    template_sections (id) {
        id -> Uuid,
        template_id -> Uuid,
        header_text_id -> Uuid,
        order_number -> Int4,
        instructions_text_id -> Uuid,
        help_text_id -> Uuid,
        character_limit -> Nullable<Int4>,
    }
}

table! {
    templates (id) {
        id -> Uuid,
        name_text_id -> Uuid,
        purpose_text_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        slug -> Varchar,
    }
}

table! {
    texts (id, lang) {
        id -> Uuid,
        section_id -> Nullable<Uuid>,
        lang -> Varchar,
        content -> Varchar,
        translated -> Bool,
        machine_translation -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Uuid,
        hash -> Bytea,
        salt -> Varchar,
        email -> Varchar,
        user_name -> Varchar,
        slug -> Varchar,
        created_at -> Timestamp,
        role -> Varchar,
        validated -> Bool,
    }
}

joinable!(sections -> documents (document_id));
joinable!(sections -> template_sections (template_section_id));
joinable!(template_sections -> templates (template_id));
joinable!(texts -> sections (section_id));

allow_tables_to_appear_in_same_query!(
    documents,
    email_verification_code,
    password_reset_token,
    sections,
    template_sections,
    templates,
    texts,
    users,
);
