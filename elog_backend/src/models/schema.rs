// ###### //
// Tables //
// ##### //

table! {
    user_role (id) {
        id -> SmallInt,
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
        id -> SmallInt,
        name -> Varchar,
        description -> Varchar,
    }
}

table! {
    user_pay_method (id) {
        id -> SmallInt,
        user_id -> SmallInt,
        pay_type_id -> SmallInt,
        bank_name -> Varchar,
        description -> Varchar,
        enabled -> Bool,
        register_at -> Timestamp,
    }
}

table! {
    user_category (id) {
        id -> SmallInt,
        user_id -> SmallInt,
        category -> Varchar,
        description -> Varchar,
    }
}

table! {
    expense (id) {
        id -> Integer,
        user_category_id -> SmallInt,
        user_pay_method_id -> SmallInt,
        amount -> Double,
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

// ############# //
// Allowed Joins //
// ############# //

joinable!(expense -> user_category (user_category_id));
joinable!(expense -> user_pay_method (user_pay_method_id));
allow_tables_to_appear_in_same_query!(expense, user_category, user_pay_method);
