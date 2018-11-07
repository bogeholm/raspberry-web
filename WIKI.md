### Database structure
## Table gpio_state
Will only be set specifically for the GPIO pins used by the program

### Diesel setup
1. diesel setup --database-url=db/raspberry-web.sqlite

# Next steps
0. <s>Read GPIO list from dotenv()</s>
0. Finish all db_setup
1. get_gpio_state();
2. gpio_in_use();
3. set_gpio_state();
4. <s>Implement r2d2 pool for database</s>
5. Actor for database?
7. Add ports to .env
8. Add actix routes