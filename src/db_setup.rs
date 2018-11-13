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
    return delimiter;
}


// Given a string (read from env_var), read into vec
pub fn parse_string_to_vec(delimiter: &str, parse_str: &str) -> Result<Vec<i32>, ParseIntError> {
    
    let vec: Result<Vec<i32>, _> = parse_str.split(&delimiter)
        .map(|x| x.parse::<i32>())
        .collect();
        
    match vec {
        Ok(parsed) => {
            info!("Parsed '{}' to {:?}", parse_str, parsed);
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
            info!("Read {}, got: {}", var_to_read, read_str);
            Ok(read_str)
        }
        Err(err) => {
            warn!("Could not read {}: {}", var_to_read, err);
            Err(err)
        }
    }
}


pub fn read_env_to_hashmap(env_keys: &Vec<&'static str>) -> HashMap<&'static str, Option<Vec<i32>>> {
    
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