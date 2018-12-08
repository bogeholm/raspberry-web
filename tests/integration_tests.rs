extern crate actix;
extern crate actix_web;
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate env_logger; 
extern crate raspberry_web;
extern crate dotenv;

use dotenv::dotenv;
//use actix::prelude::*;
//use actix_web::test::TestServer;
use diesel::{r2d2::ConnectionManager, r2d2::Pool, SqliteConnection};
use diesel_migrations::RunMigrationsError;
embed_migrations!("migrations");
use actix::prelude::*;
use raspberry_web::db::{DbExecutor};
use raspberry_web::app::{AppState, gpio_status};//, set_gpio_level};

use actix_web::http;
use actix_web::test::{TestServer};

#[macro_use]
extern crate log;

// -Create in memory database-
// -Run migrations-  
// -Change crate type to lib-
// Build App state
// Start routes
// TEST
// ... Humongous refactor

// TODO: Consider moving some functionality to db.rs
fn get_pool_after_setup() -> Result<Pool<ConnectionManager<SqliteConnection>>, RunMigrationsError> {
    let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create r2d2 pool.");
    let connection = pool.get().expect("Failed to acquire connection");
    embedded_migrations::run(&connection)?;
    Ok(pool)
}

#[test]
fn test_migrations() {
    let pool = get_pool_after_setup();
    assert_eq!(pool.is_ok(), true)
}

// Did not work, for a reason
// let srv = TestServer::with_factory(move || app::create_app(addr.clone()));

#[test]
fn get_status() {
    dotenv().ok();
    env_logger::init();
    // https://github.com/actix/actix-website/blob/master/content/docs/testing.md
    let mut srv = TestServer::build_with_state( || {
        // we can start diesel actors
        let addr = SyncArbiter::start(3, || DbExecutor(
            get_pool_after_setup().expect("Could not run migrations").clone()));
        // then we can construct custom state, or it could be `()`
        AppState{db: addr}
   })
   // register server handlers and start test server
   .start(|app| {
        app.resource(
            "/status/{id}", |r| r.method(http::Method::GET).with(gpio_status)
        );
    });
    
    let request = srv.client(http::Method::GET, "/status/2")
        .finish().unwrap();
    let response = srv.execute(request.send()).unwrap();
    info!("{:?}", response);
    assert!(response.status().is_success())

    // now we can run our test code
}

