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
use actix_web::error;
use actix_web::error::Error;
use actix_web::{App, Path, State, http, AsyncResponder, HttpResponse, FutureResponse, middleware};
use actix_web::middleware::Logger;
use models::{DbExecutor, Gpio, GetState};
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, SqliteConnection};
use dotenv::dotenv;
use std::collections::HashMap;
use std::env;
use schema::gpio_state::dsl::*;


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
    db_utilities::reset_table_gpio_state(&connection); // Will log errors / warnings

    // Read these variables from .env
    let env_keys = vec![
        "GPIOS_IN_USE",
        "GPIOS_MODE_OUTPUT",//"GPIOS_MODE_INPUT",
        "GPIOS_LEVEL_LOW",//"GPIOS_LEVEL_HIGH"
    ];

    // Parse env_keys, commit to database
    let parsed_variables = db_setup::read_env_to_hashmap(&env_keys);
    db_setup::commit_variables_to_db(&parsed_variables, &connection); // Will log errors / warnings

    let sys = actix::System::new("raspberry-web");
    // https://github.com/actix/actix-website/blob/master/content/docs/databases.md
    // https://docs.rs/actix-web/0.6.3/actix_web/struct.State.html
    let addr = SyncArbiter::start(3, move || DbExecutor(pool.clone()));

    server::new( move || {
        // TODO: Add logging
        App::with_state(AppState{db: addr.clone()})
            .middleware(middleware::Logger::default())
            .resource(
                "/state/{gpio_id}",                             // <- define path parameters
                |r| r.method(http::Method::GET).with(get_state) // <- use `with` extractor
            )
    })
        .bind(format!("{}:{}", hostname, port))
        .expect(&format!("Cannot bind to '{}:{}'", hostname, port))
        .start();
    /*
    // Test only
    let x = gpio_state
            .filter(gpio_id.eq(2))
            .load::<models::Gpio>(&connection).unwrap();
    assert!(x.len() == 1);
    println!("{:?}", x[0].gpio_level);
    */
    let _ = sys.run();
}

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

impl Message for GetState {
    type Result = Result<GetState, Error>;
}

// extract path info using serde
// TODO: Async
fn get_state(
        (state, info): (State<AppState>, Path<GetState>),
        ) -> HttpResponse {
    info!("Received {:?}", info);
    //let this_id = info[0].gpio_id;
    
    HttpResponse::Ok()
        .content_type("plain/text")
        .body(format!("{}", info.gpio_id))
}

impl Handler<GetState> for DbExecutor {
    type Result = Result<GetState, Error>;
    
    fn handle(&mut self, msg: GetState, _: &mut Self::Context) -> Self::Result {
        use self::schema::gpio_state::dsl::*;
        let conn: &SqliteConnection = &self.0.get().unwrap(); // TODO

        //let gpio_id = &msg.gpio_id;
        info!("GetState: {}", 2);
        println!("Hello!");
        let gpio_vec = gpio_state
            .filter(gpio_id.eq(&msg.gpio_id)) // TODO ;)
            .load::<models::Gpio>(conn)//.unwrap();
            .map_err(|_| error::ErrorInternalServerError("Error loading person"))?;

        Ok(GetState{gpio_id: gpio_vec[0].gpio_id})

        // match x {
        //     Ok(val) => {
        //         info!("Logging val: {}", val[0].gpio_level);
        //         Ok(GetState{gpio_id: 2})
        //     }
        //     Err(err) => {
        //         //err.into()
        //         Err(error::ErrorInternalServerError("Erroneous error message"))
        //     }
        
        //x[0].gpio_level

        //Ok(x.pop().unwrap())

        /*
        match x {
            Ok(val) => {
                val
            }
            Err(err) => {
                err
            }
        }
        */
        
        //assert!(x.len() == 1);
        //println!("{:?}", x[0].gpio_level);        

        //Ok(items.pop().unwrap())
        //Ok(GetState)
    }
}