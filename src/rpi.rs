use parking_lot::Mutex;
#[cfg(target_arch = "arm")]
use rppal::gpio::{Error::InstanceExists, Gpio};
use std::io::{Error, ErrorKind};
use std::sync::Arc;

#[cfg(not(target_arch = "arm"))]
pub type GpioArcMutex = Arc<Mutex<i32>>;

#[cfg(target_arch = "arm")]
pub type GpioArcMutex = Arc<Mutex<Gpio>>;

#[cfg(not(target_arch = "arm"))]
pub fn create_gpio_arc_mutex() -> Result<GpioArcMutex, String> {
    Ok(Arc::new(Mutex::new(0)))
}

#[cfg(target_arch = "arm")]
pub fn create_gpio_arc_mutex() -> Result<GpioArcMutex, InstanceExists> {
    Arc::new(Mutex::new(Gpio::new()))?
}

#[cfg(not(target_arch = "arm"))]
pub fn set_gpio_level_rpi(
    _gpio_num: i32,
    level: &str,
    gpio_arc_mutex: GpioArcMutex,
) -> Result<(), Error> {
    let mut data = gpio_arc_mutex.lock();
    match level {
        "high" => *data += 1,
        "low" => *data += 1,
        _ => {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Invalid level: '{}'", level),
            ))
        }
    }
    Ok(())
}

#[cfg(target_arch = "arm")]
pub fn set_gpio_level_rpi(
    gpio_num: i32,
    level: &str,
    gpio_arc_mutex: GpioArcMutex,
) -> Result<(), Error> {
    let gpio_instance = gpio_arc_mutex.lock();

    let mut gpio = gpio_instance.get(gpio_num)?.into_output();

    match level {
        "high" => {
            info!("Set gpio #{} to 'high'", gpio_num);
            gpio_instance.set_high()
            },
        "low" => {
            info!("Set gpio #{} to 'low'", gpio_num);
            gpio_instance.set_low()},
        _ => {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Invalid level: '{}'", level),
            ))
        }
    }
    Ok(())
}
/*
 #[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn set_gpio_level_rpi_known_level() {
        let res = set_gpio_level_rpi(1, "high", create_gpio_arc_mutex().unwrap());
        
    }
}
*/