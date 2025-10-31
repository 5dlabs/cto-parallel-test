// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        price -> Numeric,
        inventory_count -> Int4,
    }
}

diesel::table! {
    carts (id) {
        id -> Int4,
        user_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    cart_items (id) {
        id -> Int4,
        cart_id -> Int4,
        product_id -> Int4,
        quantity -> Int4,
    }
}

diesel::joinable!(carts -> users (user_id));
diesel::joinable!(cart_items -> carts (cart_id));
diesel::joinable!(cart_items -> products (product_id));

diesel::allow_tables_to_appear_in_same_query!(users, products, carts, cart_items,);
