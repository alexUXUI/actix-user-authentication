table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        refresh_token -> Nullable<Varchar>,
    }
}
