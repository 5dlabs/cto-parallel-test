diesel::table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    products (id) {
        id -> Integer,
        name -> Varchar,
        description -> Text,
        price -> Numeric,
        inventory_count -> Integer,
    }
}

diesel::table! {
    carts (id) {
        id -> Integer,
        user_id -> Integer,
        created_at -> Timestamp,
    }
}

diesel::table! {
    cart_items (id) {
        id -> Integer,
        cart_id -> Integer,
        product_id -> Integer,
        quantity -> Integer,
    }
}

diesel::joinable!(carts -> users (user_id));
diesel::joinable!(cart_items -> carts (cart_id));
diesel::joinable!(cart_items -> products (product_id));

diesel::allow_tables_to_appear_in_same_query!(users, products, carts, cart_items);
