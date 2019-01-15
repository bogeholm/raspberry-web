### Database structure
## Table gpio_state
Will only be set specifically for the GPIO pins used by the program

### Diesel setup
1. diesel setup --database-url=db/raspberry-web.sqlite

## To Do - General Functionality
1. <s>Read GPIO list from `.env`</s>
2. <s>Finish all db_setup</s>
3. <s>`get_gpio_state()`</s>
4. <s>`gpio_in_use()`</s>
5. <s>`set_gpio_state()`</s>
6. <s>Implement `r2d2` pool for database</s>
7. <s>Actor for database?</s>
8. <s>Add ports to `.env`</s>
9. <s>Add actix routes</s>
10. <s>Check if Gpio is set to Outout before changing</s>
11. <s>Return `Result<(), Error>` from more functions</s>
13. <s>Get list of gpio id's from DB</s>
14. <s>Unify caps in database</s>
15. <s>Split functionality into `main.rs` and `lib.rs`</s>
17. During setup, check that GPIOs where LEVEL, OUTPUT and MODE are set are also set to IN_USE
18. <s>`setup.rs` + `utilities.rs`: return `Result` </s>

## To Do - Adding `rppal` functionality
- <s>rpi.rs: `get_gpio_mutex()`</s>
- <s>rpi.rs: `set_gpio_level_rpi`</s>
- <s>app.rs: `AppState`</s>
- <s>app.rs: `create_app_state`</s>
- <s>lib.rs: `server::new...`</s>
- <s>app.rs: `pub fn set_gpio_level()`</s>
- <s>utilities.rs: `set_gpio_level`</s>
- <s>rename `db.rs` to `handlers.rs`</s>
- <s>fix integration tests</s>


## To Do - Code Quality
- <s>Fix `schema.rs` -> `Nullable<Text>`</s>
- Refactor `Handler<GpioLevel> for DbExecutor` for functionality duplication
- <s>Integration tests</s>
- Move everything with compilation configuration into rpi.rs
- <s>Rename duplicate function names in `utilities.rs`</s>

## Publication to `crates.io`
- Read good guide
- Rename to `raspberry-web`

# Version 0.2.x
## Functionality
- Check GPIO also when checking status

## Documentation
- Good `README.md`
- Add Postman collection for documenation

## DevOps
- Dockerfile based on `rust:latest`
- GitLab CI including build on ARM

## Code quality
- Better `actix` error types
- Refactor error messages, don't use `InternalServerError` all over
- Unit tests
- Check responses in integration tests
- Organize in directories
- Consistent logging strategy
- Fix `app::set_gpio_level()` double clone travesty
- More use of `pub type GpioArcMutex`