### Database structure
## Table gpio_state
Will only be set specifically for the GPIO pins used by the program

### Diesel setup
1. diesel setup --database-url=db/raspberry-web.sqlite

# To Do - General Functionality
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
12. Add Postman collection for documenation
13. <s>Get list of gpio id's from DB</s>
14. <s>Unify caps in database</s>
15. <s>Split functionality into `main.rs` and `lib.rs`</s>

# To Do - Adding `rppal` functionality
- rpi.rs: <s>`get_gpio_mutex()`</s>
- rpi.rs: <s>`set_gpio_level_rpi`</s>
- app.rs: `AppState`
- app.rs: `create_app_state`
- lib.rs: `server::new...`
- utilities.rs: `set_gpio_level`

# To Do - Code Quality
1. <s>Fix `schema.rs` -> `Nullable<Text>`</s>
2. Refactor `Handler<GpioLevel> for DbExecutor` for functionality duplication
3. Better `actix` error types
4. Refactor error messages
5. Unit tests
6. <s>Integration tests</s>
7. Check responses in integration tests