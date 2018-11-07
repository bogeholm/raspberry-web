table! {
    gpio_state (gpio_id) {
        gpio_id -> Integer,
        in_use -> Integer,
        gpio_mode -> Nullable<Text>,
        gpio_level -> Nullable<Text>,
        last_change -> Nullable<Timestamp>,
    }
}
