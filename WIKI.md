### Database structure
## Table gpio_state
Will only be set specifically for the GPIO pins used by the program

gpio: i32, primary key, not null (identifies GPIO)
in_use: i32, not null, default 0 (boolean)
mode: text (input or output)
level: text (high or low)
last_changed: text (timestamp as text of last change)

### Diesel setup
1. diesel setup --database-url=db/raspberry-web.sqlite