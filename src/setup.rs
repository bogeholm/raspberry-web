use diesel::prelude::*;
use std::collections::HashMap;
use std::env;
use std::env::VarError;
use std::num::ParseIntError;
use crate::utilities::{set_gpio_in_use, set_gpio_level, set_gpio_mode};

pub fn read_env_delimiter() -> String {
    // Get delimiter from .env - if not set, use
    let default_delimiter = ",".to_string();
    let env_delimiter = env::var("DELIMITER");
    let delimiter = match env_delimiter {
        Ok(val) => {
            debug!("Using '{}' as delimiter in .env", val);
            val
        }
        Err(_err) => {
            debug!("No delimiter set - defaulting to '{}'", default_delimiter);
            default_delimiter
        }
    };
    return delimiter;
}

// Given a string (read from env_var), read into vec
pub fn parse_string_to_vec(delimiter: &str, parse_str: &str) -> Result<Vec<i32>, ParseIntError> {
    let vec: Result<Vec<i32>, _> = parse_str
        .split(&delimiter)
        .map(|x| x.parse::<i32>())
        .collect();

    match vec {
        Ok(parsed) => {
            debug!("Parsed '{}' to {:?}", parse_str, parsed);
            Ok(parsed)
        }
        Err(err) => {
            warn!("Could not parse '{}' to Vec<i32>: {}", parse_str, err);
            Err(err)
        }
    }
}

// Read env_var into string, handle errors
pub fn read_env_to_str(var_to_read: &str) -> Result<String, VarError> {
    let env_var = env::var(var_to_read);

    match env_var {
        Ok(read_str) => {
            debug!("Read {}, got: {}", var_to_read, read_str);
            Ok(read_str)
        }
        Err(err) => {
            warn!("Could not read {}: {}", var_to_read, err);
            Err(err)
        }
    }
}

pub fn read_env_to_hashmap(env_keys: &Vec<&'static str>) -> HashMap<&'static str, Vec<i32>> {
    let delimiter = read_env_delimiter();
    let mut parsed_variables: HashMap<&'static str, Vec<i32>> = HashMap::new();

    for env_key in env_keys {
        let env_str = read_env_to_str(env_key);
        //parsed_variables.insert(env_key, None);

        if let Ok(env_var) = env_str {
            let env_vec = parse_string_to_vec(&delimiter, &env_var);

            if let Ok(parsed_vec) = env_vec {
                parsed_variables.insert(&env_key, parsed_vec);
            }
        }
    }
    return parsed_variables;
}

pub fn commit_variables_to_db(map: &HashMap<&'static str, Vec<i32>>, conn: &SqliteConnection) {
    // Should be set to 1
    match map.get("GPIOS_IN_USE") {
        Some(vec) => {
            for idx in vec.iter() {
                set_gpio_in_use(*idx, 1, conn); // Logging happens in db_utilities
            }
        }
        _ => debug!("GPIOS_IN_USE not set"),
    }

    // Should be set to OUTPUT
    match map.get("GPIOS_MODE_OUTPUT") {
        Some(vec) => {
            for idx in vec.iter() {
                set_gpio_mode(*idx, "output", conn); // Logging happens in db_utilities
            }
        }
        _ => debug!("GPIOS_MODE_OUTPUT not set"),
    }

    // Should be set to INPUT
    match map.get("GPIOS_MODE_INPUT") {
        Some(vec) => {
            for idx in vec.iter() {
                set_gpio_mode(*idx, "input", conn); // Logging happens in db_utilities
            }
        }
        _ => debug!("GPIOS_MODE_INPUT not set"),
    }

    // Should be set to LOW
    match map.get("GPIOS_LEVEL_LOW") {
        Some(vec) => {
            for idx in vec.iter() {
                set_gpio_level(*idx, "low", conn); // Logging happens in db_utilities
            }
        }
        _ => debug!("GPIOS_LEVEL_LOW not set"),
    }

    // Should be set to LOW
    match map.get("GPIOS_LEVEL_HIGH") {
        Some(vec) => {
            for idx in vec.iter() {
                set_gpio_level(*idx, "high", conn); // Logging happens in db_utilities
            }
        }
        _ => debug!("GPIOS_LEVEL_HIGH not set"),
    }
}
