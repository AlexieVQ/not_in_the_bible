table! {
    history (id) {
        id -> Varchar,
    }
}

table! {
    requests (id) {
        id -> Varchar,
        user -> Varchar,
        date -> Timestamp,
        op_id -> Varchar,
        op_author -> Varchar,
        text -> Text,
        lang -> Nullable<Varchar>,
        quoted -> Bool,
    }
}

table! {
    responses (id) {
        id -> Varchar,
        body -> Text,
        user -> Varchar,
        date -> Timestamp,
        op_id -> Varchar,
        op_author -> Varchar,
        quoted -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    history,
    requests,
    responses,
);
