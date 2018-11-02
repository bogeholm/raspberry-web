use schema::gpio_state;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "gpio_state"]
pub struct Gpio {
    pub gpio_id: i32,
    pub in_use: i32,
    pub gpio_mode: String,
    pub gpio_level: String,
    pub last_change: String
}