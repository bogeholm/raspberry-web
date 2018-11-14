use super::schema::gpio_state;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]// -> #[macro_use] extern crate serde_derive;
//#[derive(Debug, Queryable, Insertable)]
#[table_name = "gpio_state"]
pub struct Gpio {
    pub gpio_id: i32,               // 0..16 + 21..31
    pub in_use: i32,                // 0 or 1
    pub gpio_mode: String,          // INPUT or OUTPUT
    pub gpio_level: String,         // HIGH or LOW
    pub last_change: NaiveDateTime  // Timestamp
}