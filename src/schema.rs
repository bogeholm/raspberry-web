// Check out https://github.com/diesel-rs/diesel/issues/330
// TODO: Change gpio_mode, gpio_level, last_change to Option<String>
table! {
    gpio_state (gpio_id) {
        gpio_id -> Integer,
        in_use -> Integer,
        gpio_mode -> Text,
        gpio_level -> Text,
        last_change -> Text, // TODO: Make timestamo
    }
}
