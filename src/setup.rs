use crate::errors::RpWebError;
use crate::rpi::{
    set_gpio_level_rpi, 
    GpioArcMutex, 
    reset_gpio_output_pin_rpi, 
    set_reset_on_drop_false_for_output_pin_rpi};
use crate::settings::GpioConfig;
use crate::utilities::{set_gpio_in_use_db, set_gpio_level_db, set_gpio_mode_db};
use diesel::SqliteConnection;

pub fn setup_rpi_and_db(
    gpioconfig: &GpioConfig, conn: &SqliteConnection, gpio_arc_mutex: GpioArcMutex,
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

            reset_gpio_output_pin_rpi(*idx, gpio_arc_mutex.clone())?;
            set_reset_on_drop_false_for_output_pin_rpi(*idx, gpio_arc_mutex.clone())?;
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
