use std::sync::Arc;
use parking_lot::Mutex;
use std::io;
#[cfg(target_arch = "armv7")]
use rppal::{Gpio, Error::InstanceExists};

#[cfg(not(target_arch = "armv7"))]
pub struct GpioMutex {
    pub gpio_mutex: Arc<Mutex<i32>>
}

#[cfg(target_arch = "armv7")]
pub struct GpioMutex {
    pub gpio_mutex: Arc<Mutex<Gpio>>
}

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

/// Returns Arc<Mutex<Gpio>> on ARM, Arc<Mutex<i32>> otherwise
#[cfg(not(target_arch = "armv7"))]
pub fn get_gpio_mutex() -> Result<GpioMutex, io::Error> {
    let gpio = Arc::new(Mutex::new(0));
    Ok(GpioMutex {gpio_mutex: gpio})
}

/// Returns Arc<Mutex<Gpio>> on ARM, Arc<Mutex<i32>> otherwise
#[cfg(target_arch = "armv7")]
pub fn get_gpio_mutex() -> Result<GpioMutex, io::Error> {
    let gpio = Gpio::new()?;
    Ok(GpioMutex {gpio_mutex: Arc::new(Mutex::new(gpio))})
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