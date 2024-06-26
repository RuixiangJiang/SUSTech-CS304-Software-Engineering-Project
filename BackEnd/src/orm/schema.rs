// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use crate::orm::utils::sql_types::*;

    accounts (id) {
        id -> Int4,
        sustech_id -> Int4,
        #[max_length = 30]
        name -> Varchar,
        #[max_length = 48]
        email -> Varchar,
        #[max_length = 128]
        password -> Nullable<Varchar>,
        avatar -> Uuid,
        role -> Role,
        bio -> Text,
        registered_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::orm::utils::sql_types::*;

    chat_members (chat_id, account_id) {
        chat_id -> Int4,
        account_id -> Int4,
        last_read -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::orm::utils::sql_types::*;

    chat_messages (id) {
        id -> Int4,
        chat_id -> Int4,
        account_id -> Int4,
        content -> Text,
        is_deleted -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::orm::utils::sql_types::*;

    chats (id) {
        id -> Int4,
        #[max_length = 30]
        name -> Varchar,
        is_group -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::orm::utils::sql_types::*;

    comments (id) {
        id -> Int4,
        account_id -> Int4,
        event_id -> Int4,
        content -> Text,
        is_deleted -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::orm::utils::sql_types::*;

    events (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        kind -> Eventtype,
        description -> Text,
        cover -> Uuid,
        organizer_id -> Int4,
        start_at -> Timestamp,
        end_at -> Timestamp,
        venue_id -> Int4,
        location -> Point,
        tickets -> Nullable<Int4>,
        registration_deadline -> Nullable<Timestamp>,
        is_deleted -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::orm::utils::sql_types::*;

    moment_comments (id) {
        id -> Int4,
        account_id -> Int4,
        moment_id -> Int4,
        content -> Text,
        is_deleted -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::orm::utils::sql_types::*;

    moments (id) {
        id -> Int4,
        account_id -> Int4,
        content -> Text,
        is_deleted -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::orm::utils::sql_types::*;

    participation (account_id, event_id) {
        account_id -> Int4,
        event_id -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::orm::utils::sql_types::*;

    places (id) {
        id -> Int4,
        #[max_length = 30]
        name -> Varchar,
    }
}

diesel::joinable!(chat_members -> accounts (account_id));
diesel::joinable!(chat_members -> chats (chat_id));
diesel::joinable!(chat_messages -> accounts (account_id));
diesel::joinable!(chat_messages -> chats (chat_id));
diesel::joinable!(comments -> accounts (account_id));
diesel::joinable!(comments -> events (event_id));
diesel::joinable!(events -> accounts (organizer_id));
diesel::joinable!(events -> places (venue_id));
diesel::joinable!(moment_comments -> accounts (account_id));
diesel::joinable!(moment_comments -> moments (moment_id));
diesel::joinable!(moments -> accounts (account_id));
diesel::joinable!(participation -> accounts (account_id));
diesel::joinable!(participation -> events (event_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    chat_members,
    chat_messages,
    chats,
    comments,
    events,
    moment_comments,
    moments,
    participation,
    places,
);
