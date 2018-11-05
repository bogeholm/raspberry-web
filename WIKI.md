### Database structure
## Table gpio_state
Will only be set specifically for the GPIO pins used by the program

### Diesel setup
1. diesel setup --database-url=db/raspberry-web.sqlite

# Next steps
0. Read GPIO list from dotenv()
1. get_gpio_state();
2. gpio_in_use();
3. set_gpio_state();