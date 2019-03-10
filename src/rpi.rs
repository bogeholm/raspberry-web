use crate::errors::RpWebError;
use crate::utilities::i32_to_u8;
use parking_lot::Mutex;
#[cfg(target_arch = "arm")]
use rppal::gpio::Gpio;
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
pub fn create_gpio_arc_mutex() -> Result<GpioArcMutex, RpWebError> {
    let gpio = Gpio::new()?;
    Ok(Arc::new(Mutex::new(gpio)))
}

#[cfg(not(target_arch = "arm"))]
pub fn set_gpio_level_rpi(
    gpio_id: i32, level: &str, gpio_arc_mutex: GpioArcMutex,
) -> Result<(), RpWebError> {
    let _gpio_id_u8 = i32_to_u8(gpio_id)?;
    let mut data = gpio_arc_mutex.lock();
    match level {
        "high" => *data += 1,
        "low" => *data += 1,
        _ => {
            let errs = format!("Invalid level: '{}'", level);
            return Err(RpWebError::new(&errs));
        }
    }
    Ok(())
}

#[allow(unused_mut)] // output_pin needs mut but generates a warning
#[cfg(target_arch = "arm")]
pub fn set_gpio_level_rpi(
    gpio_id: i32, level: &str, gpio_arc_mutex: GpioArcMutex,
) -> Result<(), RpWebError> {
    let gpio_id_u8 = i32_to_u8(gpio_id)?;
    let data = gpio_arc_mutex.lock();

    //let mut pin = (*data).get(gpio_num)?.into_output();
    let mut pin = (*data).get(gpio_id_u8)?;
    let mut output_pin = pin.into_output();

    match level {
        "high" => {
            info!("Set gpio #{} to 'high'", gpio_id_u8);
            output_pin.set_high()
        }
        "low" => {
            info!("Set gpio #{} to 'low'", gpio_id_u8);
            output_pin.set_low()
        }
        _ => {
            let errs = format!("Invalid level: '{}'", level);
            return Err(RpWebError::new(&errs));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_gpio_level_rpi_high_must_succeed() {
        let gpio_arc_mutex = create_gpio_arc_mutex().expect("Could not acquire GPIO");
        let res = set_gpio_level_rpi(1, "high", gpio_arc_mutex);

        assert!(res.is_ok());
    }

    #[test]
    fn set_gpio_level_rpi_low_must_succeed() {
        let gpio_arc_mutex = create_gpio_arc_mutex().expect("Could not acquire GPIO");
        let res = set_gpio_level_rpi(1, "low", gpio_arc_mutex);

        assert!(res.is_ok());
    }

    #[test]
    fn set_gpio_level_rpi_unknown_must_fail() {
        let gpio_arc_mutex = create_gpio_arc_mutex().expect("Could not acquire GPIO");
        let res = set_gpio_level_rpi(1, "unknown_level", gpio_arc_mutex);

        assert!(res.is_err());
    }
}
