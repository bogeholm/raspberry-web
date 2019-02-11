use crate::errors::RpWebError;
use crate::rpi::{set_gpio_level_rpi, GpioArcMutex};
use crate::utilities::{set_gpio_in_use_db, set_gpio_level_db, set_gpio_mode_db};
use diesel::SqliteConnection;
use std::collections::HashMap;
use std::env;

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
pub fn parse_string_to_vec(delimiter: &str, parse_str: &str) -> Result<Vec<i32>, RpWebError> {
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
            Err(err)?
        }
    }
}

// Read env_var into string, handle errors
pub fn read_env_to_str(var_to_read: &str) -> Result<String, RpWebError> {
    let env_var = env::var(var_to_read);

    match env_var {
        Ok(read_str) => {
            debug!("Read {}, got: {}", var_to_read, read_str);
            Ok(read_str)
        }
        Err(err) => {
            // Should not be a warning - user can specify which variables to read
            info!("Did not find {}: {}", var_to_read, err);
            Err(err)?
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

pub fn setup_rpi_and_db(
    map: &HashMap<&'static str, Vec<i32>>,
    conn: &SqliteConnection,
    gpio_arc_mutex: GpioArcMutex,
) -> Result<(), RpWebError> {
    // Should be set to 1
    match map.get("GPIOS_IN_USE") {
        Some(vec) => {
            for idx in vec.iter() {
                let _ = set_gpio_in_use_db(*idx, 1, conn)?;
            }
        }
        _ => debug!("GPIOS_IN_USE not set"),
    }

    // Should be set to OUTPUT
    match map.get("GPIOS_MODE_OUTPUT") {
        Some(vec) => {
            for idx in vec.iter() {
                let _ = set_gpio_mode_db(*idx, "output", conn)?;
            }
        }
        _ => debug!("GPIOS_MODE_OUTPUT not set"),
    }

    // Should be set to INPUT
    match map.get("GPIOS_MODE_INPUT") {
        Some(vec) => {
            for idx in vec.iter() {
                let _ = set_gpio_mode_db(*idx, "input", conn)?;
            }
        }
        _ => debug!("GPIOS_MODE_INPUT not set"),
    }

    // Should be set to LOW
    match map.get("GPIOS_LEVEL_LOW") {
        Some(vec) => {
            for idx in vec.iter() {
                let _ = set_gpio_level_rpi(*idx, "low", gpio_arc_mutex.clone())?;
                let _ = set_gpio_level_db(*idx, "low", conn)?;
            }
        }
        _ => debug!("GPIOS_LEVEL_LOW not set"),
    }

    // Should be set to HIGH
    match map.get("GPIOS_LEVEL_HIGH") {
        Some(vec) => {
            for idx in vec.iter() {
                let _ = set_gpio_level_rpi(*idx, "high", gpio_arc_mutex.clone())?;
                let _ = set_gpio_level_db(*idx, "high", conn)?;
            }
        }
        _ => debug!("GPIOS_LEVEL_HIGH not set"),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    // If tests fail when run in parallel, and the error is not consistently reproducible,
    // it may be due to a race condition when setting DELIMITER to both "," and ";" in
    // different tests below. In that case, run in serial or remove conflicting tests

    #[test]
    fn delimiter_set_semicolon() {
        env::set_var("DELIMITER", ";");
        let res = read_env_delimiter();

        assert_eq!(";", res);
    }

    #[test]
    fn delimiter_not_set_must_return_comma() {
        env::remove_var("DELIMITER");
        let res = read_env_delimiter();

        assert_eq!(",", res);
    }

    #[test]
    fn parse_valid_string_comma_separated_to_vec_must_succeed() {
        let str_to_parse = "1,2";
        let delimiter = ",";
        let res = parse_string_to_vec(delimiter, str_to_parse);
        let expected = vec![1, 2];

        assert!(res.is_ok());
        assert_eq!(res.expect("Test failed"), expected);
    }

    #[test]
    fn parse_valid_string_semicolon_separated_to_vec_must_succeed() {
        let str_to_parse = "1;2";
        let delimiter = ";";
        let res = parse_string_to_vec(delimiter, str_to_parse);
        let expected = vec![1, 2];

        assert!(res.is_ok());
        assert_eq!(res.expect("Test failed"), expected);
    }

    #[test]
    fn parse_invalid_string_contains_letter_to_vec_must_fail() {
        let str_to_parse = "1,a";
        let delimiter = ",";
        let res = parse_string_to_vec(delimiter, str_to_parse);

        assert!(res.is_err());
    }

    #[test]
    fn parse_invalid_string_contains_float_to_vec_must_fail() {
        let str_to_parse = "1,2.3";
        let delimiter = ",";
        let res = parse_string_to_vec(delimiter, str_to_parse);

        assert!(res.is_err());
    }

    #[test]
    fn read_existing_env_var_must_succeed() {
        let key = "test-succeeds";
        let val = "test-variable";
        env::set_var(key, val);
        let res = read_env_to_str(key);

        assert!(res.is_ok());
        assert_eq!(res.expect("Test failed"), val.to_string());
    }

    #[test]
    fn read_non_existing_env_var_must_fail() {
        let key = "test-fails";
        env::remove_var(key);
        let res = read_env_to_str(key);

        assert!(res.is_err());
    }

    /// Compare two &Vec<i32>'s for elementwise equality
    fn i32vecs_equal(u: &Vec<i32>, v: &Vec<i32>) -> bool {
        //https://stackoverflow.com/questions/40767815
        (u.len() == v.len()) && u.iter().zip(v).all(|(a, b)| a == b)
    }

    #[test]
    fn read_env_to_hashmap_must_match_expected() {
        //let mut expected: HashMap<&'static str, Vec<i32>> = HashMap::new();
        // environment keys
        let key1 = "first_match";
        let key2 = "second_match";
        // environment values for env
        let env_val1 = "1,2";
        let env_val2 = "3,4";
        // expected values
        let exp_val1 = vec![1, 2];
        let exp_val2 = vec![3, 4];

        // keys to be read from env
        let read_vec: Vec<&'static str> = vec![key2, key1];

        // set up environment variables
        env::set_var("DELIMITER", ",");
        env::set_var(key1, env_val1);
        env::set_var(key2, env_val2);

        let res_map = read_env_to_hashmap(&read_vec);
        let res_val1 = res_map.get(key1).expect("Test failed");
        let res_val2 = res_map.get(key2).expect("Test failed");

        assert!(i32vecs_equal(&exp_val1, res_val1));
        assert!(i32vecs_equal(&exp_val2, res_val2));
    }

    #[test]
    fn read_env_to_hashmap_bad_values_must_fail() {
        //let mut expected: HashMap<&'static str, Vec<i32>> = HashMap::new();
        // environment keys
        let key1 = "first_none";
        let key2 = "second_none";
        // environment values for env
        let env_val1 = "1,2.0";
        let env_val2 = "3,a";

        // keys to be read from env
        let read_vec: Vec<&'static str> = vec![key2, key1];

        // set up environment variables
        env::set_var("DELIMITER", ",");
        env::set_var(key1, env_val1);
        env::set_var(key2, env_val2);

        let res_map = read_env_to_hashmap(&read_vec);

        assert!(res_map.get(key1).is_none());
        assert!(res_map.get(key2).is_none());
    }
}
