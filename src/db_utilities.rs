use diesel::prelude::*;
use schema::gpio_state::dsl::*;
use chrono::{Local};

pub fn set_gpio_state_db(id: i32, state: i32, conn: &SqliteConnection) {
    let target = gpio_state.filter(gpio_id.eq(id));
    
    // TODO: Add timestamp
    let result = diesel::update(target)
        .set((in_use.eq(state), last_change.eq(Local::now().naive_local())))
        .execute(conn);
        
    match result {
        Ok(val) => {
            if val == 1 {
                info!("Set 'in_use={}' for GPIO #{}", 1, id);
            }
            else {
                warn!{"SQL statement for GPIO #{} affects {} rows", id, val};
            }
        },
        Err(err) => error!("Failed to update GPIO #{}: {:?}", id, err),
        }
}