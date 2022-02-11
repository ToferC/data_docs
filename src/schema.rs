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

allow_tables_to_appear_in_same_query!(
    email_verification_code,
    password_reset_token,
    users,
);
