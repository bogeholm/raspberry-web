// to avoid the warning from diesel macros
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

pub mod app;
pub mod db;
mod models;
mod rpi;
pub mod schema;
mod setup;
mod utilities;

//use actix::prelude::*;
use actix::{SyncArbiter};
use actix_web::server;
use crate::db::DbExecutor;
use diesel::{r2d2::ConnectionManager, SqliteConnection};
use dotenv::dotenv;
use std::env;
use std::sync::{Arc, Mutex};


pub fn setup_and_run() {
    // Read environment variables from .env - must come before env_logger::init()
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let hostname = env::var("HOSTNAME").expect("HOSTNAME not found");
    let port = env::var("PORT").expect("PORT not found");

    // Initialize logger
    env_logger::init();

    let gpio_mutex = rpi::get_gpio_mutex().expect("Unable to acquire Gpio mutex");
    inc_mutex(gpio_mutex.gpio_mutex.clone());
    println!("{:?}", gpio_mutex.gpio_mutex);

    // Create database connection pool
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create r2d2 pool.");
    let connection = pool.get().expect("Failed to acquire connection");

    // Reset database
    utilities::reset_table_gpio_state(&connection).expect("Unable to update table 'gpio_state'");

    // Read these variables from .env
    let env_keys = vec![
        "GPIOS_IN_USE",
        "GPIOS_MODE_OUTPUT", //"GPIOS_MODE_INPUT",
        "GPIOS_LEVEL_LOW",   //"GPIOS_LEVEL_HIGH"
    ];

    // Parse env_keys, commit to database
    let parsed_variables = setup::read_env_to_hashmap(&env_keys);
    setup::commit_variables_to_db(&parsed_variables, &connection); // Will log errors / warnings

    let sys = actix::System::new("raspberry-web");
    // https://github.com/actix/actix-website/blob/master/content/docs/databases.md
    // https://docs.rs/actix-web/0.6.3/actix_web/struct.State.html
    let addr = SyncArbiter::start(3, move || DbExecutor(pool.clone()));

    let ip_port = format!("{}:{}", hostname, port);
    let _server = server::new(move || app::create_app(addr.clone()))
        .bind(&ip_port)
        .expect(&format!("Can not bind to {}", &ip_port))
        .start();

    let _ = sys.run();
}

pub fn inc_mutex (whatever: Arc<Mutex<i32>>) {
    let mut whatever = whatever.lock().unwrap();
    *whatever += 1;
}