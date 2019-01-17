use crate::rpi::{set_gpio_level_rpi, GpioArcMutex};
use crate::utilities::{set_gpio_in_use_db, set_gpio_level_db, set_gpio_mode_db};
use diesel::SqliteConnection;
use std::collections::HashMap;
use std::env;
use std::env::VarError;
use std::io::{Error, ErrorKind};
use std::num::ParseIntError;

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
            // Should not be a warning - user can specify which variables to read
            info!("Did not find {}: {}", var_to_read, err);
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

pub fn validate_setup(map: &HashMap<&'static str, Vec<i32>>) -> Result<(), Error> {
    // All GPIOs set to either 'high' or 'low' for 'gpio_level' must have 'in_use' = 1
    // All GPIOs set to either 'high' or 'low' for 'gpio_level' must have 'gpio_mode' = 'output'
    // GPIO's set to 'gpio_level' = 'low' must not be set to 'high' and vice versa
    let levels = vec!["GPIOS_LEVEL_LOW", "GPIOS_LEVEL_HIGH"];

    for level in levels.iter() {
        if let Some(vec) = map.get(*level) {
            // Must be present if levels are set
            let in_use = map.get("GPIOS_IN_USE").ok_or(Error::new(ErrorKind::Other, 
                "GPIO_LEVEL_* is set, but GPIOS_IN_USE is not set"))?;
            
            // Must be present if levels are set
            let output = map.get("GPIOS_MODE_OUTPUT").ok_or(Error::new(ErrorKind::Other, 
                "GPIO_LEVEL_* is set, but GPIOS_MODE_OUTPUT is not set"))?;
            
            for idx in vec.iter() {
                if !in_use.contains(idx) {
                    return Err(Error::new(ErrorKind::Other, 
                    format!("GPIO #{} is not IN_USE, but {} is set for it", idx, *level)
                ));}

                if !output.contains(idx) {
                    return Err(Error::new(ErrorKind::Other, 
                    format!("GPIO #{} is not configured to OUTPUT, but {} is set for it", idx, *level)
                ));}
            }
        }
    }

    if let Some(vec_low) = map.get("GPIOS_LEVEL_LOW") {
        if let Some(vec_high) = map.get("GPIOS_LEVEL_HIGH") {
            for id_low in vec_low.iter() {
                if vec_high.contains(id_low) {
                    return Err(Error::new(ErrorKind::Other, 
                    format!("GPIO #{} is in both GPIOS_LEVEL_LOW and GPIOS_LEVEL_HIGH", id_low)));
                }
            }
        }
    }

    Ok(())
}

pub fn commit_variables_to_db(
    map: &HashMap<&'static str, Vec<i32>>,
    conn: &SqliteConnection,
    gpio_arc_mutex: GpioArcMutex,
) -> Result<(), Error> {
    // Should be set to 1
    match map.get("GPIOS_IN_USE") {
        Some(vec) => {
            for idx in vec.iter() {
                let _ = set_gpio_in_use_db(*idx, 1, conn)
                    .map_err(|err| Error::new(ErrorKind::Other, err.to_string()))?;
            }
        }
        _ => debug!("GPIOS_IN_USE not set"),
    }

    // Should be set to OUTPUT
    match map.get("GPIOS_MODE_OUTPUT") {
        Some(vec) => {
            for idx in vec.iter() {
                let _ = set_gpio_mode_db(*idx, "output", conn)
                    .map_err(|err| Error::new(ErrorKind::Other, err.to_string()))?;
            }
        }
        _ => debug!("GPIOS_MODE_OUTPUT not set"),
    }

    // Should be set to INPUT
    match map.get("GPIOS_MODE_INPUT") {
        Some(vec) => {
            for idx in vec.iter() {
                let _ = set_gpio_mode_db(*idx, "input", conn)
                    .map_err(|err| Error::new(ErrorKind::Other, err.to_string()))?;
            }
        }
        _ => debug!("GPIOS_MODE_INPUT not set"),
    }

    // Should be set to LOW
    match map.get("GPIOS_LEVEL_LOW") {
        Some(vec) => {
            for idx in vec.iter() {
                let _ = set_gpio_level_rpi(*idx, "low", gpio_arc_mutex.clone())?;
                let _ = set_gpio_level_db(*idx, "low", conn)
                    .map_err(|err| Error::new(ErrorKind::Other, err.to_string()))?;
            }
        }
        _ => debug!("GPIOS_LEVEL_LOW not set"),
    }

    // Should be set to HIGH
    match map.get("GPIOS_LEVEL_HIGH") {
        Some(vec) => {
            for idx in vec.iter() {
                let _ = set_gpio_level_rpi(*idx, "high", gpio_arc_mutex.clone())?;
                let _ = set_gpio_level_db(*idx, "high", conn)
                    .map_err(|err| Error::new(ErrorKind::Other, err.to_string()))?;
            }
        }
        _ => debug!("GPIOS_LEVEL_HIGH not set"),
    }
    Ok(())
}
