use crate::errors::RpWebError;
use crate::models;
use crate::schema::gpio_state::dsl::*;
use chrono::Local;
use diesel::prelude::*;
use std::collections::HashMap;
use std::u8::{MAX, MIN};

pub fn reset_table_gpio_state(connection: &SqliteConnection) -> Result<(), RpWebError> {
    info!("Resetting all fields in table 'gpio_state'...");

    // Get all GPIO id's from table gpio_state
    use crate::schema::gpio_state::dsl::*;
    let gpio_ids_db: Vec<i32> = gpio_state
        .load::<models::Gpio>(connection)?
        .into_iter()
        .map(|element| element.gpio_id)
        .collect();

    for idx in gpio_ids_db.iter() {
        let target = gpio_state.filter(gpio_id.eq(idx));

        let n_updated = diesel::update(target)
            .set((
                in_use.eq(0),
                last_change.eq(Local::now().naive_local().to_string()),
                // These two next ones can be discussed
                gpio_mode.eq(""),
                gpio_level.eq(""),
            ))
            .execute(connection)?; // DatabaseError

        if n_updated == 1 {
            debug!("Reset values for GPIO #{}", idx);
        } else {
            let errs = format!(
                "SQL for resetting table 'gpio_state' for GPIO #{} affects {} rows",
                idx, n_updated
            );
            error!("{}", errs);
            Err(RpWebError::new(&errs))?
        }
    }
    Ok(())
}

pub fn get_allowed_states(
    connection: &SqliteConnection, desired_type: &str,
) -> Result<HashMap<&'static str, bool>, RpWebError> {
    use crate::schema::allowed_states::dsl::*;

    let res = allowed_states
        .filter(state_type.eq(desired_type.to_lowercase()))
        .load::<models::AllowedStates>(connection)? // DatabaseError
        .pop()
        .ok_or({ RpWebError::new("state_type not found") })? //Empty vector
        .to_hashmap();

    Ok(res)
}

pub fn set_gpio_in_use_db(id: i32, state: i32, conn: &SqliteConnection) -> Result<(), RpWebError> {
    let target = gpio_state.filter(gpio_id.eq(id));

    let result = diesel::update(target)
        .set((
            in_use.eq(state),
            last_change.eq(Local::now().naive_local().to_string()),
        ))
        .execute(conn);

    match result {
        Ok(val) => {
            if val == 1 {
                info!("Set 'in_use={}' for GPIO #{}", state, id);
            } else {
                let errs = format!(
                    "SQL statement 'in_use={}' for GPIO #{} affects {} rows",
                    state, id, val
                );
                Err(RpWebError::new(&errs))?;
            }
        }
        Err(err) => {
            error!(
                "Failed to update 'in_use={}' for GPIO #{}: {:?}",
                state, id, err
            );
            Err(err)?;
        }
    }
    Ok(())
}

pub fn set_gpio_mode_db(id: i32, mode: &str, conn: &SqliteConnection) -> Result<(), RpWebError> {
    let target = gpio_state.filter(gpio_id.eq(id));

    let result = diesel::update(target)
        .set((
            gpio_mode.eq(mode),
            last_change.eq(Local::now().naive_local().to_string()),
        ))
        .execute(conn);

    match result {
        Ok(val) => {
            if val == 1 {
                info!("Set 'gpio_mode={}' for GPIO #{}", mode, id);
            } else {
                let errs = format!(
                    "SQL statement 'gpio_mode={}' for GPIO #{} affects {} rows",
                    mode, id, val
                );
                Err(RpWebError::new(&errs))?;
            }
        }
        Err(err) => {
            error!(
                "Failed to update 'gpio_mode={}' for GPIO #{}: {:?}",
                mode, id, err
            );
            Err(err)?;
        }
    }
    Ok(())
}

pub fn set_gpio_level_db(id: i32, level: &str, conn: &SqliteConnection) -> Result<(), RpWebError> {
    let target = gpio_state.filter(gpio_id.eq(id));

    let result = diesel::update(target)
        .set((
            gpio_level.eq(level),
            last_change.eq(Local::now().naive_local().to_string()),
        ))
        .execute(conn);

    match result {
        Ok(val) => {
            if val == 1 {
                info!("Set 'gpio_level={}' for GPIO #{}", level, id);
            } else {
                let errs = format!(
                    "SQL statement 'gpio_level={}' for GPIO #{} affects {} rows",
                    level, id, val
                );
                Err(RpWebError::new(&errs))?;
            }
        }
        Err(err) => {
            error!(
                "Failed to update 'gpio_level={}' for GPIO #{}: {:?}",
                level, id, err
            );
            Err(err)?;
        }
    }
    Ok(())
}

/// Convert x: i32 to u8 if MIN(u8)=0 x <= x <= MAX(u8)=255
pub fn i32_to_u8(x: i32) -> Result<u8, RpWebError> {
    if i32::from(MIN) <= x && x <= i32::from(MAX) {
        Ok(x as u8)
    } else {
        let errs = format!("Not satisfied: {} <= {} <= {}", MIN, x, MAX);
        Err(RpWebError::new(&errs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn below_u8_min_must_fail() {
        assert!(i32_to_u8(MIN as i32 - 1).is_err())
    }

    #[test]
    pub fn above_u8_max_must_fail() {
        assert!(i32_to_u8(MAX as i32 + 1).is_err())
    }

    #[test]
    pub fn within_range_must_succeed() {
        let xi32: i32 = 17;
        let resu8 = i32_to_u8(xi32).unwrap();
        assert_eq!(xi32 as u8, resu8);
    }
}
