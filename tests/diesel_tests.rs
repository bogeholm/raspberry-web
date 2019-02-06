#[macro_use]
extern crate diesel_migrations;
extern crate raspberry_web;

use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, r2d2::Pool, SqliteConnection};
use diesel_migrations::RunMigrationsError;
use raspberry_web::models;
use raspberry_web::schema;
use raspberry_web::utilities::{
    get_allowed_states,
    reset_table_gpio_state,
    set_gpio_in_use_db,
    set_gpio_mode_db,
    //set_gpio_mode_level_db
};

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

#[test]
fn reset_table_gpio_state_after_update_must_succeed() {
    use crate::schema::gpio_state::dsl::*;
    let pool = get_pool_after_migrations().expect("Failed to create r2d2 pool.");
    let connection = pool.get().expect("Failed to acquire connection");

    // Variables
    let gpio_changed = 1;
    let mode = "output";
    let level = "low";

    // Change
    let _ = diesel::update(gpio_state)
        .set((in_use.eq(1), gpio_mode.eq(mode), gpio_level.eq(level)))
        .filter(gpio_id.eq(gpio_changed))
        .execute(&connection)
        .expect("Test failed");

    // Reset
    reset_table_gpio_state(&connection).expect("Test failed");

    // Check
    let gpio_reset = gpio_state
        .filter(gpio_id.eq(gpio_id))
        .load::<models::Gpio>(&connection)
        .expect("Test failed")
        .pop()
        .expect("Test failed");

    println!("{:?}", gpio_reset);

    assert_eq!(gpio_reset.in_use, 0);
    assert_eq!(gpio_reset.gpio_mode, Some("".to_string()));
    assert_eq!(gpio_reset.gpio_level, Some("".to_string()));
}

#[test]
fn set_existing_gpio_in_use_db_must_succeed() {
    use crate::schema::gpio_state::dsl::*;
    let pool = get_pool_after_migrations().expect("Failed to create r2d2 pool.");
    let connection = pool.get().expect("Failed to acquire connection");

    let gpio_id_change = 1;
    let in_use_state = 1;

    let res = set_gpio_in_use_db(gpio_id_change, in_use_state, &connection);
    assert!(res.is_ok());

    let gpio_changed = gpio_state
        .filter(gpio_id.eq(gpio_id_change))
        .load::<models::Gpio>(&connection)
        .expect("Test failed")
        .pop()
        .expect("Test failed");

    assert_eq!(gpio_changed.in_use, 1);
}

#[test]
fn set_nonexisting_gpio_in_use_db_must_fail() {
    let pool = get_pool_after_migrations().expect("Failed to create r2d2 pool.");
    let connection = pool.get().expect("Failed to acquire connection");

    let gpio_id_change = 42;
    let in_use_state = 1;

    let res = set_gpio_in_use_db(gpio_id_change, in_use_state, &connection);
    assert!(res.is_err());
}

#[test]
fn set_mode_existing_gpio_db_must_succeed() {
    use crate::schema::gpio_state::dsl::*;
    let pool = get_pool_after_migrations().expect("Failed to create r2d2 pool.");
    let connection = pool.get().expect("Failed to acquire connection");

    let gpio_id_change = 1;
    let mode_change = "input".to_string();

    let res = set_gpio_mode_db(gpio_id_change, &mode_change, &connection);
    assert!(res.is_ok());

    let gpio_changed = gpio_state
        .filter(gpio_id.eq(gpio_id_change))
        .load::<models::Gpio>(&connection)
        .expect("Test failed")
        .pop()
        .expect("Test failed");

    assert_eq!(gpio_changed.gpio_mode, Some(mode_change));
}

#[test]
fn set_mode_nonexisting_gpio_db_must_fail() {
    let pool = get_pool_after_migrations().expect("Failed to create r2d2 pool.");
    let connection = pool.get().expect("Failed to acquire connection");

    let gpio_id_change = 42;
    let mode_change = "input".to_string();

    let res = set_gpio_mode_db(gpio_id_change, &mode_change, &connection);
    assert!(res.is_err());
}

/*
#[test]
fn set_existing_gpio_to_forbidden_mode_db_must_fail() {
    assert!(true);
}
*/

#[test]
fn set_level_existing_gpio_db_must_succeed() {
    use crate::schema::gpio_state::dsl::*;
    let pool = get_pool_after_migrations().expect("Failed to create r2d2 pool.");
    let connection = pool.get().expect("Failed to acquire connection");

    let gpio_id_change = 1;
    let level_change = "input".to_string();

    let res = set_gpio_mode_db(gpio_id_change, &level_change, &connection);
    assert!(res.is_ok());

    let gpio_changed = gpio_state
        .filter(gpio_id.eq(gpio_id_change))
        .load::<models::Gpio>(&connection)
        .expect("Test failed")
        .pop()
        .expect("Test failed");

    assert_eq!(gpio_changed.gpio_mode, Some(level_change));
}

#[test]
fn set_level_nonexisting_gpio_db_must_fail() {
    let pool = get_pool_after_migrations().expect("Failed to create r2d2 pool.");
    let connection = pool.get().expect("Failed to acquire connection");

    let gpio_id_change = 42;
    let level_change = "high".to_string();

    let res = set_gpio_mode_db(gpio_id_change, &level_change, &connection);
    assert!(res.is_err());
}

/*
#[test]
fn set_existing_gpio_to_forbidden_level_db_must_fail() {
    assert!(true);
}
*/
