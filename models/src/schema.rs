// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Uuid,
        title -> Varchar,
        desc -> Nullable<Text>,
        body -> Text,
        user_id -> Uuid,
        tag_id -> Nullable<Uuid>,
        state -> Int2,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

diesel::table! {
    tags (id) {
        id -> Uuid,
        name -> Varchar,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

diesel::joinable!(posts -> tags (tag_id));
diesel::joinable!(posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    tags,
    users,
);
