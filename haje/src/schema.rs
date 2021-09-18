table! {
    accounts (id) {
        id -> Int4,
        name -> Text,
        password -> Text,
        email -> Text,
        created_on -> Timestamp,
        last_login -> Nullable<Timestamp>,
        create_ip -> Text,
        last_ip -> Nullable<Text>,
    }
}
