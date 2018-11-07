use super::schema::gpio_state;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]// -> #[macro_use] extern crate serde_derive;
//#[derive(Debug, Queryable, Insertable)]
#[table_name = "gpio_state"]
pub struct Gpio {
    pub gpio_id: i32,
    pub in_use: i32,
    pub gpio_mode: String,
    pub gpio_level: String,
    pub last_change: NaiveDateTime
}

impl Gpio {
    pub fn set_in_use(mut self, in_use_boolean: i32) -> Self {
        self.in_use = in_use_boolean;
        self
    }
}