use actix::{Actor, Handler, Message, SyncContext};
use actix_web::{error, Error};
use chrono::Local;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::models;
use crate::utilities::get_allowed_states;

//use utilities::get_allowed_states;

/// This is db executor actor. We are going to run 3 of them in parallel.
pub struct DbExecutor(pub Pool<ConnectionManager<SqliteConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub struct GpioId {
    pub gpio_id: i32,
}

impl Message for GpioId {
    type Result = Result<models::Gpio, Error>;
}

pub struct CheckGpioLevel {
    pub gpio_id: i32,
    pub gpio_level: String,
}

impl Message for CheckGpioLevel {
    type Result = Result<models::Gpio, Error>;
}

pub struct SetGpioLevel {
    pub gpio_id: i32,
    pub gpio_level: String,
}

impl Message for SetGpioLevel {
    type Result = Result<models::Gpio, Error>;
}


impl Handler<GpioId> for DbExecutor {
    type Result = Result<models::Gpio, Error>;

    fn handle(&mut self, msg: GpioId, _: &mut Self::Context) -> Self::Result {
        use crate::schema::gpio_state::dsl::*;

        let connection = &self
            .0
            .get()
            .map_err(|_| error::ErrorInternalServerError("Error obtaining database connection"))?;

        let mut gpio_vec = gpio_state
            .filter(gpio_id.eq(msg.gpio_id))
            .load::<models::Gpio>(connection)
            .map_err(|_| error::ErrorInternalServerError("Error loading from database"))?;

        gpio_vec.pop().ok_or({
            error::ErrorInternalServerError(format!(
                "No result for GPIO #{} in database",
                msg.gpio_id
            ))
        })
    }
}

impl Handler<CheckGpioLevel> for DbExecutor {
    type Result = Result<models::Gpio, Error>;

    fn handle(&mut self, msg: CheckGpioLevel, _: &mut Self::Context) -> Self::Result {
        let required_gpio_mode = "output";
        use crate::schema::gpio_state::dsl::*;
        let connection = &self
            .0
            .get()
            .map_err(|_| error::ErrorInternalServerError("Error obtaining database connection"))?;

        // 1. Load Vec<Gpio> from database
        let gpio_before = gpio_state
            .filter(gpio_id.eq(msg.gpio_id))
            .load::<models::Gpio>(connection)
            .map_err(|_| error::ErrorInternalServerError("Error loading from database"))?
            .pop()
            .ok_or({
                error::ErrorInternalServerError(format!(
                    "No result for GPIO #{} in database",
                    msg.gpio_id
                ))
            })?;

        // 2. Check if the GPIO is in use
        let bool_in_use = gpio_before.in_use == 1;
        if !bool_in_use {
            info!("GPIO #{} not in use.", msg.gpio_id);
            return Err(error::ErrorInternalServerError(format!(
                "GPIO #{} not in use.",
                msg.gpio_id
            )));
        }
        
        // 3. check if gpio_mode = 'output'
        let none_replacement = "".to_string();
        // https://stackoverflow.com/questions/22282117/how-do-i-borrow-a-reference-to-what-is-inside-an-optiont
        let gpio_mode_before = gpio_before.gpio_mode.as_ref().unwrap_or(&none_replacement);
        if gpio_mode_before != required_gpio_mode {
            let message = format!(
                "Level '{}' is not allowed for mode '{}'",
                msg.gpio_level, gpio_mode_before
            );
            info!("{}", message);
            return Err(error::ErrorInternalServerError(message));
        }
        
        // 4. Check if 'msg.gpio_level' is allowed
        let desired_level = msg.gpio_level.to_lowercase();
        let state_map = get_allowed_states(connection, "level")
            .map_err(|_| error::ErrorInternalServerError("Error loading from database"))?;

        let allowed =
            state_map
                .get::<str>(&desired_level)
                .ok_or(error::ErrorInternalServerError(format!(
                    "Could not find level '{}' in table 'allowed_states'",
                    desired_level
                )))?;
        if !allowed {
            info!("Level '{}' is not allowed", desired_level);
            Err(error::ErrorInternalServerError("State not allowed"))?
        }

        Ok(gpio_before)
    }
}

impl Handler<SetGpioLevel> for DbExecutor {
    type Result = Result<models::Gpio, Error>;

    fn handle(&mut self, msg: SetGpioLevel, _: &mut Self::Context) -> Self::Result {
        use crate::schema::gpio_state::dsl::*;
        let connection = &self
            .0
            .get()
            .map_err(|_| error::ErrorInternalServerError("Error obtaining database connection"))?;

        // 5. Change the level
        let target = gpio_state.filter(gpio_id.eq(msg.gpio_id));

        let _result = diesel::update(target)
            .set((
                last_change.eq(Local::now().naive_local().to_string()),
                gpio_level.eq(msg.gpio_level.to_lowercase()),
            ))
            .execute(connection);

        // 6. Return Gpio state after update
        let mut gpio_vec_after = gpio_state
            .filter(gpio_id.eq(msg.gpio_id))
            .load::<models::Gpio>(connection)
            .map_err(|_| error::ErrorInternalServerError("Error loading from database"))?;

        gpio_vec_after.pop().ok_or(
            // At this point we msg.gpio.id is in db
            error::ErrorInternalServerError("Could not connect to database"),
        )
    }
}