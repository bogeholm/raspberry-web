use crate::errors::RpWebError;
use crate::rpi::{set_gpio_level_rpi, GpioArcMutex};
use crate::settings::GpioConfig;
use crate::utilities::{set_gpio_in_use_db, set_gpio_level_db, set_gpio_mode_db};
use diesel::SqliteConnection;

pub fn setup_rpi_and_db(
    gpioconfig: &GpioConfig,
    conn: &SqliteConnection,
    gpio_arc_mutex: GpioArcMutex,
) -> Result<(), RpWebError> {
    
    // Should be set to 1
    if let Some(gpios_in_use) = &gpioconfig.gpios_in_use {
        for idx in gpios_in_use.iter() {
            let _ = set_gpio_in_use_db(*idx, 1, conn)?;
        }
    }

    // Should be set to OUTPUT
    if let Some(gpios_mode_output) = &gpioconfig.gpios_mode_output {
        for idx in gpios_mode_output.iter() {
            let _ = set_gpio_mode_db(*idx, "output", conn)?;
        }
    }

    // Should be set to INPUT
    if let Some(gpios_mode_input) = &gpioconfig.gpios_mode_input {
        for idx in gpios_mode_input.iter() {
            let _ = set_gpio_mode_db(*idx, "input", conn)?;
        }
    }

    // Should be set to LOW
    if let Some(gpios_level_low) = &gpioconfig.gpios_level_low {
        for idx in gpios_level_low.iter() {
            let _ = set_gpio_level_rpi(*idx, "low", gpio_arc_mutex.clone())?;
            let _ = set_gpio_level_db(*idx, "low", conn)?;
        }
    }

    // Should be set to HIGH
    if let Some(gpios_level_high) = &gpioconfig.gpios_level_high {
        for idx in gpios_level_high.iter() {
            let _ = set_gpio_level_rpi(*idx, "high", gpio_arc_mutex.clone())?;
            let _ = set_gpio_level_db(*idx, "high", conn)?;
        }
    }

    Ok(())
}

/*
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
*/
