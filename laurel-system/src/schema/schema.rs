diesel::table! {
    use diesel::sql_types::*;

    fe_micro_service (id) {
        id -> Int8,
        #[max_length = 40]
        app_id -> Varchar,
        #[max_length = 40]
        service_id -> Varchar,
        #[max_length = 64]
        service_name -> Varchar,
        #[max_length = 64]
        service_entry -> Varchar,
        #[max_length = 64]
        mount_point -> Varchar,
        #[max_length = 100]
        route_pattern -> Varchar,
        #[max_length = 20]
        service_status -> Varchar,
        cts -> Timestamp,
        uts -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    menu(id){
        id -> Int8,
        #[max_length = 40]
        app_id -> Varchar,
        #[max_length = 40]
        menu_id -> Varchar,
        #[max_length = 64]
        menu_name -> Varchar,
        #[max_length = 40]
        menu_type -> Varchar,
        #[max_length = 40]
        menu_action_type -> Varchar,
        #[max_length = 60]
        menu_icon -> Nullable<Varchar>,
        #[max_length = 400]
        menu_route -> Nullable<Varchar>,
        route_param -> Nullable<Text>,
        weight -> Int4,
        #[max_length = 40]
        parent_id -> Varchar,
        #[max_length = 100]
        authority -> Nullable<Varchar>,
        #[max_length = 20]
        menu_status -> Varchar,
        cts -> Timestamp,
        uts -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    role(id){
        id -> Int8,
        #[max_length = 40]
        role_id -> Varchar,
        #[max_length = 64]
        role_name -> Varchar,
        #[max_length = 40]
        role_type -> Varchar,
        weight -> Int4,
        #[max_length = 20]
        role_status -> Varchar,
        cts -> Timestamp,
        uts -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    dict(id){
        id -> Int8,
        #[max_length = 64]
        dict_id -> Varchar,
        #[max_length = 200]
        dict_name -> Varchar,
        dict_mark -> Nullable<Text>,
        weight -> Int4,
        #[max_length = 40]
        dict_type -> Varchar,
        cts -> Timestamp,
        uts -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    dict_value(id){
        id -> Int8,
        #[max_length = 64]
        dict_id -> Varchar,
        #[max_length = 64]
        value_id -> Varchar,
        #[max_length = 200]
        value_name -> Varchar,
        value_mark -> Nullable<Text>,
        weight -> Int4,
        #[max_length = 40]
        dict_type -> Varchar,
        cts -> Timestamp,
        uts -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(micro_app,);
diesel::allow_tables_to_appear_in_same_query!(menu,);
diesel::allow_tables_to_appear_in_same_query!(role,);
diesel::allow_tables_to_appear_in_same_query!(dict,);
diesel::allow_tables_to_appear_in_same_query!(dict_value,);
