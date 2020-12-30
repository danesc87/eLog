table! {
    user_role (id) {
        id -> TinyInt,
        description -> Varchar,
    }
}

table! {
    app_user (id) {
        id -> SmallInt,
        first_name -> Varchar,
        last_name -> Varchar,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        register_at -> Timestamp,
    }
}

table! {
    pay_type (id) {
        id -> TinyInt,
        name -> Varchar,
        description -> Varchar,
    }
}

table! {
    user_pay_method (id) {
        id -> TinyInt,
        user_id -> SmallInt,
        pay_type_id -> TinyInt,
        bank_name -> Varchar,
        description -> Varchar,
        enabled -> Bool,
        register_at -> Timestamp,
    }
}

table! {
    expense (id) {
        id -> Integer,
        user_pay_method_id -> TinyInt,
        ammount -> Double,
        description -> Varchar,
        register_at -> Timestamp,
    }
}

table! {
    invalid_tokens (string_token) {
        string_token -> Text,
        expiration_date -> Timestamp,
    }
}
