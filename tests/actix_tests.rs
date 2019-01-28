#[macro_use]
extern crate diesel_migrations;
extern crate log;
extern crate raspberry_web;

use actix::SyncArbiter;
use actix_web::http;
use actix_web::test::TestServer;
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, r2d2::Pool, SqliteConnection};
use diesel_migrations::RunMigrationsError;
use dotenv::dotenv;
use std::sync::{Once, ONCE_INIT};

use raspberry_web::app::{gpio_status_route, set_gpio_level_route, AppState};
use raspberry_web::handlers::DbExecutor;
use raspberry_web::rpi::create_gpio_arc_mutex;
use raspberry_web::schema;

embed_migrations!("migrations");

static INIT: Once = ONCE_INIT;

/// Initialize logging only once
fn init_logging_once() {
    INIT.call_once(|| {
        dotenv().ok();
        env_logger::init();
    });
}

/// Create r2d2 pool, run diesel migrations
fn get_pool_after_migrations(
) -> Result<Pool<ConnectionManager<SqliteConnection>>, RunMigrationsError> {
    let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create r2d2 pool.");
    let connection = pool.get().expect("Failed to acquire connection");
    embedded_migrations::run(&connection)?;
    Ok(pool)
}

/// Setup test database for integration tests
fn setup_db_for_tests(connection: &SqliteConnection) -> Result<(), diesel::result::Error> {
    // http://diesel.rs/guides/all-about-inserts/
    use crate::schema::gpio_state::dsl::*;

    // gpio #1: can be polled and updated
    diesel::update(gpio_state)
        .set((in_use.eq(1), gpio_mode.eq("output"), gpio_level.eq("low")))
        .filter(gpio_id.eq(1))
        .execute(connection)?;

    // gpio #2: not in use
    diesel::update(gpio_state)
        .set(in_use.eq(0))
        .filter(gpio_id.eq(2))
        .execute(connection)?;

    // gpio #3: in use, mode is input
    diesel::update(gpio_state)
        .set((in_use.eq(1), gpio_mode.eq("input")))
        .filter(gpio_id.eq(3))
        .execute(connection)?;

    Ok(())
}

// Did not work, for a reason:
// let test_server = TestServer::with_factory(move || app::create_app(addr.clone()));

/// Build test server with state, setup db for tests and return testserver
fn get_testserver_with_state() -> TestServer {
    init_logging_once();
    // https://github.com/actix/actix-website/blob/master/content/docs/testing.md
    let test_server = TestServer::build_with_state(|| {
        let gpio_arc_mutex = create_gpio_arc_mutex().expect("Could not acquire GPIO");
        // we can start diesel actors
        let addr = SyncArbiter::start(3, || {
            DbExecutor({
                let pool = get_pool_after_migrations().expect("Could not run migrations");
                let connection = pool.get().expect("Failed to acquire connection");
                setup_db_for_tests(&connection).expect("Error setting up test database");
                pool.clone()
            })
        });
        // then we can construct custom state, or it could be `()`
        AppState {
            db: addr.clone(),
            gpio_arc_mutex: gpio_arc_mutex.clone(),
        }
    })
    // register server handlers and start test server
    .start(|app| {
        app.resource("/status/{id}", |r| {
            r.method(http::Method::GET).with(gpio_status_route)
        })
        .resource("/set/level/{id}/{level}", |r| {
            r.method(http::Method::GET).with(set_gpio_level_route)
        });
    });
    test_server
}

#[test]
fn test_migrations() {
    let pool = get_pool_after_migrations();
    assert!(pool.is_ok())
}

#[test]
fn check_status_succes() {
    // given
    let mut test_server = get_testserver_with_state();

    // when
    let request = test_server
        .client(http::Method::GET, "/status/1")
        .finish()
        .unwrap();
    let response = test_server.execute(request.send()).unwrap();

    // then
    assert!(response.status().is_success())
}

#[test]
fn check_status_gpio_nonexistant_failure() {
    // given
    let mut test_server = get_testserver_with_state();

    // when
    let request = test_server
        .client(http::Method::GET, "/status/18")
        .finish()
        .unwrap();
    let response = test_server.execute(request.send()).unwrap();

    // then
    assert_eq!(response.status(), http::StatusCode::NOT_FOUND)
}

#[test]
fn set_gpio_level_success() {
    // given
    let mut test_server = get_testserver_with_state();

    // when
    let request = test_server
        .client(http::Method::GET, "/set/level/1/high")
        .finish()
        .unwrap();
    let response = test_server.execute(request.send()).unwrap();

    // then
    assert!(response.status().is_success())
}

#[test]
fn set_gpio_level_gpio_nonexistant_failure() {
    // given
    let mut test_server = get_testserver_with_state();

    // when
    let request = test_server
        .client(http::Method::GET, "/set/level/18/high")
        .finish()
        .unwrap();
    let response = test_server.execute(request.send()).unwrap();

    // then
    assert_eq!(response.status(), http::StatusCode::NOT_FOUND)
}

#[test]
fn set_gpio_level_gpio_not_in_use_failure() {
    // given
    let mut test_server = get_testserver_with_state();

    // when
    let request = test_server
        .client(http::Method::GET, "/set/level/2/high")
        .finish()
        .unwrap();
    let response = test_server.execute(request.send()).unwrap();

    // then
    assert_eq!(response.status(), http::StatusCode::FORBIDDEN)
}

#[test]
fn set_gpio_level_gpio_mode_not_output_failure() {
    // given
    let mut test_server = get_testserver_with_state();

    // when
    let request = test_server
        .client(http::Method::GET, "/set/level/3/high")
        .finish()
        .unwrap();
    let response = test_server.execute(request.send()).unwrap();

    // then
    assert_eq!(response.status(), http::StatusCode::FORBIDDEN)
}

#[test]
fn set_gpio_level_unknown_level_failure() {
    // given
    let mut test_server = get_testserver_with_state();

    // when
    let request = test_server
        .client(http::Method::GET, "/set/level/1/something_random")
        .finish()
        .unwrap();
    let response = test_server.execute(request.send()).unwrap();

    // then
    assert_eq!(response.status(), http::StatusCode::NOT_FOUND)
}
