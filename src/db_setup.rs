use diesel::prelude::*;
use std::env;
use schema::gpio_state::dsl::*;
use db_utilities::*;


pub fn establish_connection(database_url: &str) -> Result<SqliteConnection, ConnectionError> {
    let connection = SqliteConnection::establish(database_url)?;
        Ok(connection)
}

pub fn read_env_delimiter() -> String {
    // Get delimiter from .env - if not set, use 
    let default_delimiter = ",".to_string();
    let env_delimiter = env::var("DELIMITER");
    let delimiter = match env_delimiter {
        Ok(val) => {
            info!("Using '{}' as delimiter in .env", val);
            val
            },
        Err(err) => {
            info!("No delimiter set - defaulting to '{}'", default_delimiter);
            default_delimiter
        }
    };
    delimiter
}


pub fn read_env_setup_database(conn: &SqliteConnection) {
    let delimiter = read_env_delimiter();

    // TODO: should not panic
    let gpios_used = env::var("GPIOS_IN_USE").expect("No GPIOs initialized");

    

    // TODO: Create separate function
    let vec = gpios_used.split(&delimiter)
        .map(|x| x.parse::<i32>().expect("EVIL"))
        .collect::<Vec<i32>>();

    for idx in vec {
        //let updated_row = diesel::update(gpio_state.filter(gpio_id.eq(idx)))
        //    .set(in_use.eq(1));
        set_gpio_state_db(idx, 1, conn);        
    }
}