use super::schema::gpio_state;
use chrono::NaiveDateTime;

// https://github.com/actix/examples/tree/master/diesel
use actix::prelude::*;
use actix_web::*;
use diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use models;
use schema;

// db executor
pub struct DbExecutor(pub Pool<ConnectionManager<SqliteConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "gpio_state"]
pub struct Gpio {
    pub gpio_id: i32,               // 0..16 + 21..31
    pub in_use: i32,                // 0 or 1
    pub gpio_mode: String,          // INPUT or OUTPUT
    pub gpio_level: String,         // HIGH or LOW
    pub last_change: String         // Timestamp
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "gpio_state"]
pub struct GetState {
    pub gpio_id: i32,
}

impl Gpio {
    pub fn set_in_use(self) -> bool {
        self.in_use == 1
    }
}