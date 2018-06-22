table! {
    keys (id) {
        id -> Text,
        key -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    languages (id) {
        id -> Text,
        code -> Text,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        parent_id -> Nullable<Text>,
    }
}

table! {
    tokens (id) {
        id -> Text,
        kind -> Text,
        token -> Text,
        created_at -> Timestamp,
        user_id -> Text,
    }
}

table! {
    translation_requests (id) {
        id -> Text,
        translation -> Text,
        comment -> Text,
        language_id -> Text,
        key_id -> Text,
        creator_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    translations (id) {
        id -> Text,
        version -> Integer,
        translation -> Text,
        comment -> Nullable<Text>,
        language_id -> Text,
        key_id -> Text,
        creator_id -> Text,
        approver_id -> Text,
        created_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Text,
        role -> Text,
        email -> Text,
        username -> Text,
        password_hash -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

joinable!(tokens -> users (user_id));
joinable!(translation_requests -> keys (key_id));
joinable!(translation_requests -> languages (language_id));
joinable!(translation_requests -> users (creator_id));
joinable!(translations -> keys (key_id));
joinable!(translations -> languages (language_id));

allow_tables_to_appear_in_same_query!(
    keys,
    languages,
    tokens,
    translation_requests,
    translations,
    users,
);
