diesel::table! {
    use diesel::sql_types::*;

    login_log (id) {
        id -> Int8,
        #[max_length = 40]
        ticket_id -> Varchar,
        #[max_length = 64]
        account -> Varchar,
        #[max_length = 20]
        login_type -> Varchar,
        #[max_length = 20]
        login_state -> Varchar,
        #[max_length = 200]
        login_result -> Nullable<Varchar>,
        #[max_length = 64]
        ip -> Nullable<Varchar>,
        #[max_length = 128]
        location -> Nullable<Varchar>,
        #[max_length = 256]
        browser -> Nullable<Varchar>,
        #[max_length = 256]
        os -> Nullable<Varchar>,
        #[max_length = 256]
        device -> Nullable<Varchar>,
        cts -> Timestamp,
        login_cts -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(login_log,);
