use crate::errors::RpWebError;
use crate::settings::GpioConfig;
//use std::collections::HashMap;

/// Return a copy of the vec in Option(vec), or an empty vector for None
pub fn vec_option_to_vec(option: &Option<Vec<i32>>) -> Vec<i32> {
    match option {
        Some(vec) => vec.to_vec(),
        None => vec![],
    }
}

/// Return Some(vec) of elements in both u and v, else None
fn elements_in_both_vecs(u: &Vec<i32>, v: &Vec<i32>) -> Option<Vec<i32>> {
    let mut res = vec![];

    // Iterate and push duplicate values
    for &idx in u.iter() {
        if v.contains(&idx) {
            res.push(idx);
        }
    }

    // Return None if res is empty
    if !res.is_empty() {
        return Some(res);
    } else {
        return None;
    }
}

pub fn validate_setup(gpioconfig: &GpioConfig) -> Result<(), RpWebError> {
    // All GPIOs set to either 'high' or 'low' for 'gpio_level' must have 'in_use' = 1
    // All GPIOs set to either 'high' or 'low' for 'gpio_level' must have 'gpio_mode' = 'output'
    // GPIO's set to 'gpio_level' = 'low' must not be set to 'high' and vice versa
    // GPIO's set to 'gpio_mode' = 'output' must not be set to 'input'

    // It will be clearer to work with vecs than options
    let gpios_in_use = vec_option_to_vec(&gpioconfig.gpios_in_use);
    let gpios_mode_output = vec_option_to_vec(&gpioconfig.gpios_mode_output);
    let gpios_mode_input = vec_option_to_vec(&gpioconfig.gpios_mode_input);
    let gpios_level_low = vec_option_to_vec(&gpioconfig.gpios_level_low);
    let gpios_level_high = vec_option_to_vec(&gpioconfig.gpios_level_high);

    // Build vector of all gpios_level_{high / low}
    let mut gpio_all_levels: Vec<i32> = vec![];
    gpio_all_levels.append(&mut gpios_level_low.to_vec());
    gpio_all_levels.append(&mut gpios_level_high.to_vec());

    // 'gpios_in_use' must be present if levels are set
    if gpios_in_use.is_empty() {
        if !gpio_all_levels.is_empty() {
            return Err(RpWebError::new(
                "Invalid configuration: gpio_levels_* is set, but gpios_in_use is empty",
            ));
        }
    }
    // 'gpios_mode_output' be present if levels are set
    if gpios_mode_output.is_empty() {
        if !gpio_all_levels.is_empty() {
            return Err(RpWebError::new(
                "Invalid configuration: gpios_level_* is set, but gpios_mode_output is empty",
            ));
        }
    }

    // Find misconfigured gpios
    for idx in gpio_all_levels.iter() {
        // for all gpios set to level high or low, they must be in use
        if !gpios_in_use.contains(idx) {
            let errs = format!(
                "Invalid configuration: GPIO #{} is not in_use, but a level is set for it",
                idx
            );
            return Err(RpWebError::new(&errs));
        }

        // for all gpios set to level high or low, they must be set to mode output
        if !gpios_mode_output.contains(idx) {
            let errs = format!(
                "Invalid configuration: GPIO #{} is not configured to OUTPUT, but a level is set for it", idx);
            return Err(RpWebError::new(&errs));
        }
    }

    // Find gpios in both level high and low, if any
    let both_high_and_low_option = elements_in_both_vecs(&gpios_level_low, &gpios_level_high);
    if let Some(high_and_low) = both_high_and_low_option {
        let errs = format!(
            "Invalid configuration: GPIO(s) {:?} in both gpios_level_low and gpios_level_high",
            high_and_low
        );
        return Err(RpWebError::new(&errs));
    }

    // Find gpios in mode_input and mode_output, if any
    let both_input_and_output_option = elements_in_both_vecs(&gpios_mode_input, &gpios_mode_output);
    if let Some(input_and_output) = both_input_and_output_option {
        let errs = format!(
            "Invalid configuration: GPIO(s) {:?} in both gpios_mode_input and gpios_mode_output",
            input_and_output
        );
        return Err(RpWebError::new(&errs));
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_in_use_none_but_level_set_must_fail() {
        let gpioconfig = GpioConfig {
            gpios_in_use: None,
            gpios_mode_output: Some(vec![1]),
            gpios_mode_input: None,
            gpios_level_low: Some(vec![1]),
            gpios_level_high: None,
        };

        let res = validate_setup(&gpioconfig);
        assert!(res.is_err());
    }

    #[test]
    fn validation_in_use_not_set_but_level_set_must_fail() {
        let gpioconfig = GpioConfig {
            gpios_in_use: Some(vec![2]),
            gpios_mode_output: Some(vec![2]),
            gpios_mode_input: None,
            gpios_level_low: Some(vec![1]),
            gpios_level_high: None,
        };

        let res = validate_setup(&gpioconfig);
        assert!(res.is_err());
    }


    #[test]
    fn validation_mode_output_none_but_level_set_must_fail() {
        let gpioconfig = GpioConfig {
            gpios_in_use: Some(vec![1]),
            gpios_mode_output: None, 
            gpios_mode_input: None,
            gpios_level_low: Some(vec![1]),
            gpios_level_high: None,
        };

        let res = validate_setup(&gpioconfig);
        assert!(res.is_err());
    }
    
    #[test]
    fn validation_mode_output_not_set_but_level_set_must_fail() {
        let gpioconfig = GpioConfig {
            gpios_in_use: Some(vec![1]),
            gpios_mode_output: Some(vec![2]),
            gpios_mode_input: None,
            gpios_level_low: Some(vec![1]),
            gpios_level_high: None,
        };

        let res = validate_setup(&gpioconfig);
        assert!(res.is_err());
    }
    
    /*
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
    */
}
