// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (account_id) {
        account_id -> Int4,
        email -> Text,
        account_type -> Text,
        account_name -> Text,
    }
}

diesel::table! {
    categories (category_id) {
        category_id -> Int4,
        email -> Text,
        nickname -> Text,
        category_type -> Text,
        budget -> Float8,
        budget_freq -> Text,
    }
}

diesel::table! {
    transactions (trans_id) {
        trans_id -> Int4,
        email -> Text,
        category_id -> Int4,
        amount -> Float8,
        notes -> Nullable<Text>,
        account_id -> Int4,
        transaction_date -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Text,
        password -> Text,
        username -> Text,
    }
}

diesel::joinable!(transactions -> accounts (account_id));
diesel::joinable!(transactions -> categories (category_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    categories,
    transactions,
    users,
);
