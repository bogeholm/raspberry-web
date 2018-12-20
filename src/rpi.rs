use std::sync::{Arc, Mutex};
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