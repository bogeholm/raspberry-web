// to avoid the warning from diesel macros
#![allow(proc_macro_derive_resolution_fallback)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate chrono;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate env_logger;
#[macro_use] extern crate log;
//extern crate serde;
#[macro_use] extern crate serde_derive;
//use log::Level;

mod schema;
mod models;
mod db_setup;
mod db_utilities;

use diesel::prelude::*;
use dotenv::dotenv;

use std::env;
//use models::Gpio;
//use schema::gpio_state::dsl::*;

fn main() {
    // Read environment variables from .env - must come before env_logger::init()
    dotenv().ok();
    
    // Initialize logger
    env_logger::init();
    
    // Get database connection
    // TODO: Create connection pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let connection = db_setup::establish_connection(&database_url)
        .expect("Could not connect to database");
    
    // Read environment variables into database
    db_setup::read_env_setup_database(&connection);
}










