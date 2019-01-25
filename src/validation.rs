use std::collections::HashMap;
use std::io::{Error, ErrorKind};

pub fn validate_setup(map: &HashMap<&'static str, Vec<i32>>) -> Result<(), Error> {
    // All GPIOs set to either 'high' or 'low' for 'gpio_level' must have 'in_use' = 1
    // All GPIOs set to either 'high' or 'low' for 'gpio_level' must have 'gpio_mode' = 'output'
    // GPIO's set to 'gpio_level' = 'low' must not be set to 'high' and vice versa
    // GPIO's set to 'gpio_mode' = 'output' must not be set to 'input'
    let levels = vec!["GPIOS_LEVEL_LOW", "GPIOS_LEVEL_HIGH"];

    for level in levels.iter() {
        if let Some(vec) = map.get(*level) {
            // Must be present if levels are set
            let in_use = map.get("GPIOS_IN_USE").ok_or(Error::new(
                ErrorKind::Other,
                "GPIO_LEVEL_* is set, but GPIOS_IN_USE is not set",
            ))?;

            // Must be present if levels are set
            let output = map.get("GPIOS_MODE_OUTPUT").ok_or(Error::new(
                ErrorKind::Other,
                "GPIO_LEVEL_* is set, but GPIOS_MODE_OUTPUT is not set",
            ))?;

            for idx in vec.iter() {
                if !in_use.contains(idx) {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("GPIO #{} is not IN_USE, but {} is set for it", idx, *level),
                    ));
                }

                if !output.contains(idx) {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!(
                            "GPIO #{} is not configured to OUTPUT, but {} is set for it",
                            idx, *level
                        ),
                    ));
                }
            }
        }
    }

    if let Some(vec_low) = map.get("GPIOS_LEVEL_LOW") {
        if let Some(vec_high) = map.get("GPIOS_LEVEL_HIGH") {
            for id_low in vec_low.iter() {
                if vec_high.contains(id_low) {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!(
                            "GPIO #{} is in both GPIOS_LEVEL_LOW and GPIOS_LEVEL_HIGH",
                            id_low
                        ),
                    ));
                }
            }
        }
    }

    if let Some(vec_input) = map.get("GPIOS_MODE_INPUT") {
        if let Some(vec_output) = map.get("GPIOS_MODE_OUTPUT") {
            for id_low in vec_input.iter() {
                if vec_output.contains(id_low) {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!(
                            "GPIO #{} is in both GPIOS_MODE_INPUT and GPIOS_MODE_OUTPUT",
                            id_low
                        ),
                    ));
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_use_not_set_but_level_set_must_fail() {
        let mut map: HashMap<&'static str, Vec<i32>> = HashMap::new();
        // Insert only level and mode, not IN_USE
        map.insert("GPIOS_MODE_OUTPUT", vec![1, 2]);
        map.insert("GPIOS_LEVEL_LOW", vec![1, 2]);

        let res = validate_setup(&map);
        assert!(res.is_err());
    }

    #[test]
    fn mode_output_not_set_but_level_set_must_fail() {
        let mut map: HashMap<&'static str, Vec<i32>> = HashMap::new();
        map.insert("GPIOS_IN_USE", vec![1, 2]);
        map.insert("GPIOS_LEVEL_HIGH", vec![1, 2]);

        let res = validate_setup(&map);
        assert!(res.is_err());
    }

    #[test]
    fn in_use_not_set_for_pin_level_set_must_fail() {
        let mut map: HashMap<&'static str, Vec<i32>> = HashMap::new();
        map.insert("GPIOS_IN_USE", vec![1, 2]);
        map.insert("GPIOS_MODE_OUTPUT", vec![1, 2, 3]);
        map.insert("GPIOS_LEVEL_LOW", vec![3]);

        let res = validate_setup(&map);
        assert!(res.is_err());
    }

    #[test]
    fn mode_output_not_set_for_pin_level_high_set_must_fail() {
        let mut map: HashMap<&'static str, Vec<i32>> = HashMap::new();
        map.insert("GPIOS_IN_USE", vec![1, 2, 3]);
        map.insert("GPIOS_MODE_OUTPUT", vec![1, 2]);
        map.insert("GPIOS_LEVEL_HIGH", vec![3]);

        let res = validate_setup(&map);
        assert!(res.is_err());
    }

    #[test]
    fn same_pin_high_and_low_must_fail() {
        let mut map: HashMap<&'static str, Vec<i32>> = HashMap::new();
        map.insert("GPIOS_IN_USE", vec![1, 2]);
        map.insert("GPIOS_MODE_OUTPUT", vec![1, 2]);
        map.insert("GPIOS_LEVEL_LOW", vec![1]);
        map.insert("GPIOS_LEVEL_HIGH", vec![1]);

        let res = validate_setup(&map);
        assert!(res.is_err());
    }

    #[test]
    fn same_pin_input_and_output_must_fail() {
        let mut map: HashMap<&'static str, Vec<i32>> = HashMap::new();
        map.insert("GPIOS_IN_USE", vec![1, 2]);
        map.insert("GPIOS_MODE_OUTPUT", vec![1]);
        map.insert("GPIOS_MODE_INPUT", vec![1]);

        let res = validate_setup(&map);
        assert!(res.is_err());
    }

    #[test]
    fn valid_setup_must_succeed() {
        let mut map: HashMap<&'static str, Vec<i32>> = HashMap::new();
        map.insert("GPIOS_IN_USE", vec![1, 2, 3]);
        map.insert("GPIOS_MODE_OUTPUT", vec![1, 2, 3]);
        map.insert("GPIOS_LEVEL_LOW", vec![1]);
        map.insert("GPIOS_LEVEL_HIGH", vec![2, 3]);

        let res = validate_setup(&map);
        assert!(res.is_ok());
    }
}
