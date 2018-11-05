// to avoid the warning from diesel macros
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
#[macro_use] 
extern crate log;
//extern crate serde;

//use log::Level;


mod schema;
mod models;

use diesel::prelude::*;
use dotenv::dotenv;
use schema::gpio_state::dsl::*;
use std::env;
//use models::Gpio;
//use schema::gpio_state::dsl::*;

#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]
fn main() {
    // Read environment variables from .env
    dotenv().ok();
    
    env_logger::init();

    let connection = establish_connection();

    // TODO: should not panic
    let gpios_used = env::var("GPIOS_IN_USE").expect("No GPIOs initialized");

    // TODO: define delimiter in .env
    // TODO: Create separate function
    let vec = gpios_used.split(",")
        .map(|x| x.parse::<i32>().expect("EVIL"))
        .collect::<Vec<i32>>();

    for idx in vec {
        //let updated_row = diesel::update(gpio_state.filter(gpio_id.eq(idx)))
        //    .set(in_use.eq(1));
        println!("Attempting to update GPIO #{}", idx);

        let target = gpio_state.filter(gpio_id.eq(idx));
        
        let result = diesel::update(target).set(in_use.eq(1))
            .execute(&connection);
        
        match result {
            Ok(v) => {
                if v == 1 {
                    info!("Set 'in_use={}' for GPIO #{}", 1, idx);
                }
                else {
                    warn!{"SQL statement for GPIO #{} affects {} rows", idx, v};
                }
            },
            Err(e) => error!("Failed to update GPIO #{}: {:?}", idx, e),
        }
    }
        
}









pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
