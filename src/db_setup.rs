use diesel::prelude::*;
use std::env;
use schema::gpio_state::dsl::*;
use db_utilities::*;
use std::io::Error as ioError;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::env::VarError;

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

pub struct EnvVariable{
    key: &'static str,
    value: Option<Vec<i32>>,
}

pub struct GpioSetup {
    pub in_use: Option<Vec<i32>>,
    pub mode_output: Vec<i32>,
    pub mode_input: Vec<i32>,
    pub level_low: Vec<i32>,
    pub level_high: Vec<i32>,
}

// Read in_use          < OK
// Read gpio_mode
// Read gpio_level
// Read into struct?

// Given a string (read from env_var), read into vec
pub fn parse_string_to_vec(delimiter: &str, parse_str: &str) -> Result<Vec<i32>, ParseIntError> {
    
    // https://users.rust-lang.org/t/error-handling-and-iterator-map-collect/4313
    // You can actually collect to a Result<Vec<u8>, _> and skip the .ok().expect(...) part.
    let vec: Result<Vec<i32>, _> = parse_str.split(&delimiter)
        .map(|x| x.parse::<i32>())
        .collect();
        vec
}

// Read env_var into string, handle errors
pub fn read_env_to_str(var_to_read: &str) -> Result<String, VarError> {
    let env_var = env::var(var_to_read);
    
    if let Ok(read_str) = env_var {
        info!("Reading environment variable {}: {}", var_to_read, read_str);
    }
   
}


pub fn read_env_to_hashmap() -> HashMap<&'static str, Option<Vec<i32>>> {
    // Read keys from .env, split at delimiter and create hashmap
    let env_keys = vec![
        "GPIOS_IN_USE",
        "GPIOS_MODE_OUTPUT",
        "GPIOS_MODE_INPUT",
        "GPIOS_LEVEL_LOW",
        "GPIOS_LEVEL_HIGH"
    ];
    let delimiter = read_env_delimiter();
    let mut parsed_variables: HashMap<&'static str, Option<Vec<i32>>> = HashMap::new();

    for env_key in env_keys {
        let env_str = read_env_to_str(env_key);
        parsed_variables.insert(env_key, None);

        if let Ok(env_var) = env_str {
            let env_vec = parse_string_to_vec(&delimiter, &env_var);

            if let Ok(parsed_vec) = env_vec{
                parsed_variables.insert(&env_key, Some(parsed_vec));
            }   
        }
    }

    return parsed_variables;
}

// TODO: Tests