### Database structure
## Table gpio_state
Will only be set specifically for the GPIO pins used by the program

### Diesel setup
1. diesel setup --database-url=db/raspberry-web.sqlite

## To Do - General Functionality
- <s>Read GPIO list from `.env`</s>
- <s>Finish all db_setup</s>
- <s>`get_gpio_state()`</s>
- <s>`gpio_in_use()`</s>
- <s>`set_gpio_state()`</s>
- <s>Implement `r2d2` pool for database</s>
- <s>Actor for database?</s>
- <s>Add ports to `.env`</s>
- <s>Add actix routes</s>
- <s>Check if Gpio is set to Outout before changing</s>
- <s>Return `Result<(), Error>` from more functions</s>
- <s>Get list of gpio id's from DB</s>
- <s>Unify caps in database</s>
- <s>Split functionality into `main.rs` and `lib.rs`</s>
- During setup, check that GPIOs where LEVEL, OUTPUT and MODE are set are also set to IN_USE
- <s>`setup.rs` + `utilities.rs`: return `Result` </s>

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
- <s>Refactor `Handler<GpioLevel> for DbExecutor` for functionality duplication</s>
- <s>Integration tests</s>
- <s>Move everything with compilation configuration into rpi.rs</s>
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