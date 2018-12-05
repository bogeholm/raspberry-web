use diesel::prelude::*;
use schema::gpio_state::dsl::*;
use chrono::{Local};
use models;
use std::collections::HashMap;

pub fn reset_table_gpio_state(conn: &SqliteConnection) -> Result<(), diesel::result::Error> {
    info!("Resetting all fields in table 'gpio_state'...");
    // TODO: get all id's from table
    let gpio_array: [i32; 28] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 
        // GPIO 17 - 20 do not exist
        21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];

    
    for idx in gpio_array.iter() {
        let target = gpio_state.filter(gpio_id.eq(idx));

        let n_updated = diesel::update(target)
            .set((
                in_use.eq(0),
                last_change.eq(Local::now().naive_local().to_string()),
                gpio_mode.eq(""),
                gpio_level.eq("")
                ))
            .execute(conn)?; // DatabaseError
            
        if n_updated == 1 {
            debug!("Reset values for GPIO #{}", idx);
        }
        else {
            error!{"SQL for resetting table 'gpio_state' for GPIO #{} affects {} rows", idx, n_updated};
            Err(diesel::result::Error::NotFound)?
        }
    }
    Ok(())
}

pub fn get_allowed_states(connection: &SqliteConnection, desired_type: &str) 
    -> Result<HashMap<&'static str, bool>, diesel::result::Error> {
    use schema::allowed_states::dsl::*;
    
    let res = allowed_states
        .filter(state_type.eq(desired_type.to_lowercase()))
        .load::<models::AllowedStates>(connection)? // DatabaseError
        .pop()
        .ok_or({diesel::result::Error::NotFound})? //Empty vector
        .to_hashmap();
    
    Ok(res)
}

// Consider returning result
pub fn set_gpio_in_use(id: i32, state: i32, conn: &SqliteConnection) {
    let target = gpio_state.filter(gpio_id.eq(id));
    
    let result = diesel::update(target)
        .set((in_use.eq(state), 
            last_change.eq(Local::now().naive_local().to_string()
            )))
        .execute(conn);
        
    match result {
        Ok(val) => {
            if val == 1 {
                info!("Set 'in_use={}' for GPIO #{}", state, id);
            }
            else {
                warn!{"SQL statement 'in_use={}' for GPIO #{} affects {} rows", state, id, val};
            }
        },
        Err(err) => error!("Failed to update 'in_use={}' for GPIO #{}: {:?}", state,  id, err),
        }
}

// Consider returning result
pub fn set_gpio_mode(id: i32, mode: &str, conn: &SqliteConnection) {
    let target = gpio_state.filter(gpio_id.eq(id));
    
    let result = diesel::update(target)
        .set((gpio_mode.eq(mode), 
            last_change.eq(Local::now().naive_local().to_string()
            )))
        .execute(conn);
        
    match result {
        Ok(val) => {
            if val == 1 {
                info!("Set 'gpio_mode={}' for GPIO #{}", mode, id);
            }
            else {
                warn!{"SQL statement 'gpio_mode={}' for GPIO #{} affects {} rows", mode, id, val};
            }
        },
        Err(err) => error!("Failed to update 'gpio_mode={}' for GPIO #{}: {:?}", mode, id, err),
        }
}

// Consider returning result
pub fn set_gpio_level(id: i32, level: &str, conn: &SqliteConnection) {
    let target = gpio_state.filter(gpio_id.eq(id));
    
    let result = diesel::update(target)
        .set((gpio_level.eq(level), 
            last_change.eq(Local::now().naive_local().to_string()
        )))
        .execute(conn);
        
    match result {
        Ok(val) => {
            if val == 1 {
                info!("Set 'gpio_level={}' for GPIO #{}", level, id);
            }
            else {
                warn!{"SQL statement 'gpio_level={}' for GPIO #{} affects {} rows", level, id, val};
            }
        },
        Err(err) => error!("Failed to update 'gpio_level={}' for GPIO #{}: {:?}", level, id, err),
        }
}


// TODO: Testing
// https://github.com/diesel-rs/diesel/blob/master/diesel_tests/tests/select.rs