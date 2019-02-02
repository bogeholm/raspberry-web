#[macro_use]
extern crate diesel_migrations;
extern crate raspberry_web;

use diesel::{r2d2::ConnectionManager, r2d2::Pool, SqliteConnection};
use diesel_migrations::RunMigrationsError;
use raspberry_web::utilities::get_allowed_states;

embed_migrations!("migrations");

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

#[test]
fn test_migrations() {
    let pool = get_pool_after_migrations();
    assert!(pool.is_ok())
}

#[test]
fn get_allowed_modes_must_succeed() {
    // Test the modes that were set up after migrations
    let pool = get_pool_after_migrations().expect("Failed to create r2d2 pool.");
    let connection = pool.get().expect("Failed to acquire connection");

    let res = get_allowed_states(&connection, "mode");
    assert!(res.is_ok());
    let map = res.expect("Test failed");

    let high = map.get(&"high").expect("Test failed");
    let low = map.get(&"low").expect("Test failed");
    let input = map.get(&"input").expect("Test failed");
    let output = map.get(&"output").expect("Test failed");

    assert_eq!(high, &false);
    assert_eq!(low, &false);
    assert_eq!(input, &true);
    assert_eq!(output, &true);
}

#[test]
fn get_allowed_levels_must_succeed() {
    // Test the levels that were set up after migrations
    let pool = get_pool_after_migrations().expect("Failed to create r2d2 pool.");
    let connection = pool.get().expect("Failed to acquire connection");

    let res = get_allowed_states(&connection, "level");
    assert!(res.is_ok());
    let map = res.expect("Test failed");

    let high = map.get(&"high").expect("Test failed");
    let low = map.get(&"low").expect("Test failed");
    let input = map.get(&"input").expect("Test failed");
    let output = map.get(&"output").expect("Test failed");

    assert_eq!(high, &true);
    assert_eq!(low, &true);
    assert_eq!(input, &false);
    assert_eq!(output, &false);
}
