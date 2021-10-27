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

table! {
    game_servers (id) {
        id -> Int4,
        name -> Text,
        ip -> Text,
        port -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(accounts, game_servers,);
