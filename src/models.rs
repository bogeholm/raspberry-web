use super::schema::{allowed_states, gpio_state};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "gpio_state"]
pub struct Gpio {
    pub gpio_id: i32,                // 0..16 + 21..31
    pub in_use: i32,                 // 0 or 1
    pub gpio_mode: Option<String>,   // INPUT or OUTPUT
    pub gpio_level: Option<String>,  // HIGH or LOW
    pub last_change: Option<String>, // Timestamp
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "allowed_states"]
pub struct AllowedStates {
    pub state_id: i32,
    pub state_type: String, // MODE or LEVEL
    // i32 for true / false below
    pub input: i32,
    pub output: i32,
    pub high: i32,
    pub low: i32,
}

impl AllowedStates {
    pub fn to_hashmap(&self) -> HashMap<&'static str, bool> {
        let mut hashed: HashMap<&'static str, bool> = HashMap::new();
        hashed.insert("input", self.input == 1);
        hashed.insert("output", self.output == 1);
        hashed.insert("high", self.high == 1);
        hashed.insert("low", self.low == 1);
        hashed
    }
}
