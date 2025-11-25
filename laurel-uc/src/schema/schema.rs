// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;

    account (id) {
        id -> Int8,
        #[max_length = 40]
        account_id -> Varchar,
        #[max_length = 64]
        account_name -> Varchar,
        #[max_length = 64]
        account_state -> Varchar,
        #[max_length = 20]
        account_type -> Varchar,
        cts -> Timestamp,
        uts -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    passport (id) {
        id -> Int8,
        #[max_length = 40]
        account_id -> Varchar,
        #[max_length = 64]
        salt -> Varchar,
        #[max_length = 64]
        password -> Varchar,
        cts -> Timestamp,
        uts -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    profile (id) {
        id -> Int8,
        #[max_length = 40]
        account_id -> Varchar,
        #[max_length = 40]
        profile_key -> Varchar,
        profile_value -> Nullable<Text>,
        cts -> Timestamp,
        uts -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(account, passport, profile,);
