//! Db executor actor
use actix::prelude::*;
use actix_web::{error, Error};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use chrono::{Local};
use models;
use utilities::get_allowed_states;

//use utilities::get_allowed_states;

/// This is db executor actor. We are going to run 3 of them in parallel.
pub struct DbExecutor(pub Pool<ConnectionManager<SqliteConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub struct GpioId { // Call it GetState
    pub gpio_id: i32
}

impl Message for GpioId {
    type Result = Result<models::Gpio, Error>;
}

pub struct GpioLevel {
    pub gpio_id: i32,
    pub gpio_level: String
}

impl Message for GpioLevel {
    type Result = Result<models::Gpio, Error>;
}

impl Handler<GpioId> for DbExecutor {
    type Result = Result<models::Gpio, Error>;

    fn handle(&mut self, msg: GpioId, _: &mut Self::Context) -> Self::Result {
        use schema::gpio_state::dsl::*;
        info!("Received {}", msg.gpio_id);

        let connection = &self.0.get()
            .map_err(|_| error::ErrorInternalServerError("Error obtaining database connection"))?;


        let mut gpio_vec = gpio_state
            .filter(gpio_id.eq(msg.gpio_id))
            .load::<models::Gpio>(connection)
            .map_err(|_| error::ErrorInternalServerError("Error loading from database"))?;

        gpio_vec.pop().ok_or({
            info!("No result for GPIO #{} in database", msg.gpio_id);
            error::ErrorInternalServerError(
                format!("No result for GPIO #{} in database", msg.gpio_id)
        )})
    }
}

impl Handler<GpioLevel> for DbExecutor {
    type Result = Result<models::Gpio, Error>;

    fn handle(&mut self, msg: GpioLevel, _: &mut Self::Context) -> Self::Result {
        use schema::gpio_state::dsl::*;
        let connection = &self.0.get()
            .map_err(|_| error::ErrorInternalServerError("Error obtaining database connection"))?;

        // 1. Load Gpio from database
        let mut gpio_vec_before = gpio_state
            .filter(gpio_id.eq(msg.gpio_id))
            .load::<models::Gpio>(connection)
            .map_err(|_| error::ErrorInternalServerError("Error loading from database"))?;

        // 2. Check if the GPIO is in use
        let bool_in_use = gpio_vec_before.pop()
            .ok_or({
                error::ErrorInternalServerError(
                    format!("No result for GPIO #{} in database", msg.gpio_id)
                )})?
            .in_use == 1;

        // TODO: 2.5 check if gpio_mode = OUTPUT

        // 3. Check if 'msg.gpio_level' is allowed
        let desired_level = msg.gpio_level.to_lowercase();
        let state_map = get_allowed_states(connection, "level")
            .map_err(|_| error::ErrorInternalServerError("Error loading from database"))?;
        
        let allowed = state_map.get::<str>(&desired_level)
            .ok_or(error::ErrorInternalServerError("Error loading from database"))?;
        if ! allowed {
                info!("Level '{}' is not allowed", desired_level);
                Err(error::ErrorInternalServerError("State not allowed"))?
        }

        // 4. Change the level
        if bool_in_use {
            let target = gpio_state.filter(gpio_id.eq(msg.gpio_id));

            let _result = diesel::update(target)
                .set((
                    last_change.eq(Local::now().naive_local().to_string()),
                    gpio_level.eq(msg.gpio_level)
                ))
            .execute(connection);
        }
        else {
            info!("GPIO #{} not in use.", msg.gpio_id);
            return Err(error::ErrorInternalServerError(
                format!("GPIO #{} not in use.", msg.gpio_id)
            ))
        }

        // 5. Return Gpio state after update
        let mut gpio_vec_after = gpio_state
            .filter(gpio_id.eq(msg.gpio_id))
            .load::<models::Gpio>(connection)
            .map_err(|_| error::ErrorInternalServerError("Error loading from database"))?;

        gpio_vec_after.pop().ok_or(
            // At this point we msg.gpio.id is in db
            error::ErrorInternalServerError("Could not connect to database"))
    }
}