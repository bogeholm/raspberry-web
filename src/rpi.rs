use std::sync::Arc;
use parking_lot::Mutex;
#[cfg(target_arch = "armv7")]
use rppal::{Gpio, Error::InstanceExists};

/// Returns Arc<Mutex<Gpio>> on ARM, Arc<Mutex<i32>> otherwise
#[cfg(target_arch = "armv7")]
pub fn get_gpio_mutex() -> Result<GpioMutex, io::Error> {
    let try_gpio = Gpio::new();

    match try_gpio {
        Ok(gpio) => {
            let gpio_mutex = Arc::new(Mutex::new(gpio));
            Ok(GpioMutex {gpio_mutex: gpio})
        }
        Err(error) => {
            // https://doc.rust-lang.org/std/io/struct.Error.html#method.new
            Err(io::Error::new(InstanceExists, error))
        }
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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