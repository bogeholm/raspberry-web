// to avoid the warning from diesel macros
#![allow(proc_macro_derive_resolution_fallback)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate chrono;
#[macro_use] 
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
#[macro_use] 
extern crate log;
#[macro_use] extern crate serde_derive;
extern crate r2d2;

mod schema;
mod models;
mod db_setup;
mod db_utilities;

use diesel::{r2d2::ConnectionManager, SqliteConnection};
use dotenv::dotenv;
use std::env;
use std::collections::HashMap;

fn main() {
    // Read environment variables from .env - must come before env_logger::init()
    dotenv().ok();
    
    // Initialize logger
    env_logger::init();
    
    // Create database connection pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create r2d2 pool.");
    let connection = pool.get().expect("Failed to acquire connection");

    // Read environment variables into database
    //db_setup::read_env_setup_database(&connection);
    db_utilities::reset_table_gpio_state(&connection);

    let env_keys = vec![
        "GPIOS_IN_USE",
        "GPIOS_MODE_OUTPUT",//"GPIOS_MODE_INPUT",
        "GPIOS_LEVEL_LOW",//"GPIOS_LEVEL_HIGH"
    ];

    let parsed_variables = db_setup::read_env_to_hashmap(&env_keys);

    db_setup::commit_variables_to_db(&parsed_variables, &connection);

}
