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

        let connection = &self.0.get()
            .map_err(|_| error::ErrorInternalServerError("Error obtaining database connection"))?;


        let mut gpio_vec = gpio_state
            .filter(gpio_id.eq(msg.gpio_id))
            .load::<models::Gpio>(connection)
            .map_err(|_| error::ErrorInternalServerError("Error loading from database"))?;

        gpio_vec.pop().ok_or({
            error::ErrorInternalServerError(
                format!("No result for GPIO #{} in database", msg.gpio_id)
        )})
    }
}

impl Handler<GpioLevel> for DbExecutor {
    type Result = Result<models::Gpio, Error>;

    fn handle(&mut self, msg: GpioLevel, _: &mut Self::Context) -> Self::Result {
        let required_gpio_mode = "output";
        use schema::gpio_state::dsl::*;
        let connection = &self.0.get()
            .map_err(|_| error::ErrorInternalServerError("Error obtaining database connection"))?;

        // 1. Load Vec<Gpio> from database
        let gpio_before = gpio_state
            .filter(gpio_id.eq(msg.gpio_id))
            .load::<models::Gpio>(connection)
            .map_err(|_| error::ErrorInternalServerError("Error loading from database"))?
            .pop()
            .ok_or({
                error::ErrorInternalServerError(
                    format!("No result for GPIO #{} in database", msg.gpio_id)
                )})?;

        // 2. Check if the GPIO is in use
        let bool_in_use = gpio_before.in_use == 1;
        if ! bool_in_use {
            info!("GPIO #{} not in use.", msg.gpio_id);
            return Err(error::ErrorInternalServerError(
                format!("GPIO #{} not in use.", msg.gpio_id)
            ))
        }

        // 3. check if gpio_mode = 'output'
        let gpio_mode_before = gpio_before.gpio_mode.unwrap_or("".to_string());
        if  gpio_mode_before != required_gpio_mode {
            let message = format!("Level '{}' is not allowed for mode '{}'", msg.gpio_level, gpio_mode_before);
            info!("{}", message);
            return Err(error::ErrorInternalServerError(message))
        }

        // TODO: Functionality duplication in 3. and 4.

        // 4. Check if 'msg.gpio_level' is allowed
        let desired_level = msg.gpio_level.to_lowercase();
        let state_map = get_allowed_states(connection, "level")
            .map_err(|_| error::ErrorInternalServerError("Error loading from database"))?;
        
        let allowed = state_map.get::<str>(&desired_level)
            .ok_or(error::ErrorInternalServerError(
                format!("Could not level '{}' in table 'allowed_states'", desired_level)
            ))?;
        if ! allowed {
                info!("Level '{}' is not allowed", desired_level);
                Err(error::ErrorInternalServerError("State not allowed"))?
        }

        // 5. Change the level
        let target = gpio_state.filter(gpio_id.eq(msg.gpio_id));

        let _result = diesel::update(target)
            .set((
                last_change.eq(Local::now().naive_local().to_string()),
                gpio_level.eq(msg.gpio_level.to_lowercase())
            ))
            .execute(connection);

        // 6. Return Gpio state after update
        let mut gpio_vec_after = gpio_state
            .filter(gpio_id.eq(msg.gpio_id))
            .load::<models::Gpio>(connection)
            .map_err(|_| error::ErrorInternalServerError("Error loading from database"))?;

        gpio_vec_after.pop().ok_or(
            // At this point we msg.gpio.id is in db
            error::ErrorInternalServerError("Could not connect to database"))
    }
}