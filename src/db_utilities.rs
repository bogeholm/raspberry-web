use diesel::prelude::*;
use schema::gpio_state::dsl::*;
use chrono::{Local};

// Consider returning result
pub fn set_gpio_in_use(id: i32, state: i32, conn: &SqliteConnection) {
    let target = gpio_state.filter(gpio_id.eq(id));
    
    let result = diesel::update(target)
        .set((in_use.eq(state), last_change.eq(Local::now().naive_local())))
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
        .set((gpio_mode.eq(mode), last_change.eq(Local::now().naive_local())))
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
        .set((gpio_level.eq(level), last_change.eq(Local::now().naive_local())))
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