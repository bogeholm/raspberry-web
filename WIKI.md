### Database structure
## Table gpio_state
Will only be set specifically for the GPIO pins used by the program

### Diesel setup
1. diesel setup --database-url=db/raspberry-web.sqlite

# To Do - Functionality
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
11. Return `Result<(), Error>` from more functions
12. Add `rppal` functionality
13. Add Postman collection for documenation
14. Get list of gpio id's from DB
15. <s>Unify caps in database</s>

# To Do - Code Quality
1. Fix `schema.rs` -> `Nullable<Text>`
2. Better `actix` errors
3. Custom error types
4. Unit tests
5. Integration tests