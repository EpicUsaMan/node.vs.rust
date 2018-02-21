table! {
    news (id) {
        id -> Serial,
        title -> VarChar,
        text -> Text,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
    }
}
