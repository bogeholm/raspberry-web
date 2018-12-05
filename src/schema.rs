table! {
    allowed_states (state_id) {
        state_id -> Integer,
        state_type -> Text,
        input -> Integer,
        output -> Integer,
        high -> Integer,
        low -> Integer,
    }
}

table! {
    gpio_state (gpio_id) {
        gpio_id -> Integer,
        in_use -> Integer,
        gpio_mode -> Text, // TODO: Nullable<Text>
        gpio_level -> Text, // TODO: Nullable<Text>
        last_change -> Text, // TODO: Make timestamp, TODO: Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    allowed_states,
    gpio_state,
);
