#[macro_use]
extern crate diesel_migrations;
extern crate raspberry_web;

use diesel::{r2d2::ConnectionManager, r2d2::Pool, SqliteConnection};
use diesel_migrations::RunMigrationsError;

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