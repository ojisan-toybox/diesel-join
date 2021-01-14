table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        image -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);

joinable!(posts -> users(user_id));