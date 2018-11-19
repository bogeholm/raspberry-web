// to avoid the warning from diesel macros
#![allow(proc_macro_derive_resolution_fallback)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate actix;
extern crate actix_web;
extern crate chrono;
#[macro_use] 
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
#[macro_use] 
extern crate log;
#[macro_use] extern crate serde_derive;
extern crate r2d2;

mod app;
mod schema;
mod models;
mod db_setup;
mod db_utilities;

use actix::prelude::*;
use actix_web::server;
use actix_web::{App, Path, State, http, HttpResponse};
use models::DbExecutor;
use diesel::{r2d2::ConnectionManager, SqliteConnection};
use dotenv::dotenv;
use std::collections::HashMap;
use std::env;


fn main() {
    // Read environment variables from .env - must come before env_logger::init()
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let hostname = env::var("HOSTNAME").expect("HOSTNAME not found");
    let port = env::var("PORT").expect("PORT not found");

    // Initialize logger
    env_logger::init();

    // Create database connection pool
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create r2d2 pool.");
    let connection = pool.get().expect("Failed to acquire connection");

    // Reset database
    db_utilities::reset_table_gpio_state(&connection);

    // Read these variables from .env
    let env_keys = vec![
        "GPIOS_IN_USE",
        "GPIOS_MODE_OUTPUT",//"GPIOS_MODE_INPUT",
        "GPIOS_LEVEL_LOW",//"GPIOS_LEVEL_HIGH"
    ];

    // Parse env_keys, commit to database
    let parsed_variables = db_setup::read_env_to_hashmap(&env_keys);
    db_setup::commit_variables_to_db(&parsed_variables, &connection);

    let sys = actix::System::new("raspberry-web");
    // https://github.com/actix/actix-website/blob/master/content/docs/databases.md
    // https://docs.rs/actix-web/0.6.3/actix_web/struct.State.html
    let addr = SyncArbiter::start(2, move || DbExecutor(pool.clone()));

    server::new( move || {
        App::with_state(AppState{db: addr.clone()}).resource(
        "/state/{id}",                      // <- define path parameters
        |r| r.method(http::Method::GET).with(index)) // <- use `with` extractor
    })
         .bind(format!("{}:{}", hostname, port))
         .expect(&format!("Cannot bind to '{}:{}'", hostname, port))
         .start();

    let _ = sys.run();


    
}

/// Application state
pub struct AppState {
    pub db: Addr<DbExecutor>,
}

#[derive(Deserialize)]
struct Info {
    id: String,
}

/// extract path info using serde
fn index(state: State<AppState>, info: Path<Info>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("plain/text")
        .body(format!("{}", info.id))
}