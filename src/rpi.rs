use std::sync::Arc;
use parking_lot::Mutex;
#[cfg(target_arch = "armv7")]
use rppal::{Gpio, Error::InstanceExists};

#[cfg(not(target_arch = "armv7"))]
pub fn set_gpio_level_rpi(_gpio_num: i32, level: &str, gpio_mutex: Arc<Mutex<i32>>) -> Result<(), String> {
    let mut data = gpio_mutex.lock();
    match level {
        "high" => {*data += 1},
        "low" => {*data += 1},
        _ => {return Err(format!("Invalid level: '{}'", level))}
    }
    Ok(())
}

#[cfg(target_arch = "armv7")]
pub fn set_gpio_level_rpi(gpio_num: i32, level: &str, gpio_mutex: Arc<Mutex<Gpio>>) -> Result<(), String> {
    let gpio_instance = gpio_mutex.lock();
    
    let mut gpio = gpio_instance.get(gpio_num)?
        .into_output();
    
    match level {
        "high" => {gpio_instance.set_high()},
        "low" => {gpio_instance.set_low()},
        _ => {return Err(format!("Invalid level: '{}'", level))}
    }
    Ok(())
}