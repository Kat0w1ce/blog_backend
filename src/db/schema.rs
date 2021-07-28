table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        path -> Text,
        intro -> Nullable<Text>,
        time -> Timestamp,
    }
}
