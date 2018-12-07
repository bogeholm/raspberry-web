extern crate actix;
extern crate actix_web;
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate raspberry_web;

//use actix::prelude::*;
//use actix_web::test::TestServer;
use diesel::{r2d2::ConnectionManager, SqliteConnection};
embed_migrations!("migrations");

//use super::db::{DbExecutor};

#[test]
fn test_test(){
    assert_eq!(1, 1);
}

// -Create in memory database-
// -Run migrations-  
// Build App state
// Start routes
// TEST
// ... Humongous refactor



#[test]
fn test_db () {
    let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create r2d2 pool.");
    let connection = pool.get().expect("Failed to acquire connection");
    let res = embedded_migrations::run(&connection);

    assert_eq!(res.is_ok(), true);
    
    //let addr = SyncArbiter::start(3, move || DbExecutor(pool.clone()));
    //let _server = server::new(move || app::create_app(addr.clone()));

}