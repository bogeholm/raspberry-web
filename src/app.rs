use std::sync::Arc;
use parking_lot::Mutex;
use actix::{Addr};
use actix_web::{error, Error};
use actix_web::{http, middleware, App, AsyncResponder, FutureResponse, HttpResponse, Path, State};
use futures::Future;
use crate::handlers::{DbExecutor, GpioId, CheckGpioLevel, SetGpioLevel};
//use crate::rpi::set_gpio_level_rpi;

#[cfg(target_arch = "armv7")]
use rppal::gpio::{Gpio, Error::InstanceExists};

/// State with DbExecutor address
#[cfg(not(target_arch = "armv7"))]
pub struct AppState {
    pub db: Addr<DbExecutor>,
    pub gpio_arc_mutex: Arc<Mutex<i32>>,
}

/// State with DbExecutor address
#[cfg(target_arch = "armv7")]
pub struct AppState {
    pub db: Addr<DbExecutor>,
    pub gpio_arc_mutex: Arc<Mutex<iGpio>>,
}

#[cfg(not(target_arch = "armv7"))]
pub fn create_gpio_arc_mutex() -> Result<Arc<Mutex<i32>>, String> {
    Ok(Arc::new(Mutex::new(0)))
}

#[cfg(target_arch = "armv7")]
pub fn create_gpio_arc_mutex() -> Result<Arc<Mutex<i32>>, InstanceExists> {
    Arc::new(Mutex::new(Gpio::new()))?
}

/// Get status of GPIO
pub fn gpio_status((req, state): (Path<i32>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(GpioId {
            gpio_id: req.into_inner(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

/// Set GPIO level to HIGH or LOW
pub fn set_gpio_level(
    (req, state): (Path<(i32, String)>, State<AppState>), // TODO: Extract multiple elements from path
) -> FutureResponse<HttpResponse> {
    let gpio_id = req.0;
    let gpio_level = req.1.clone();

    // https://github.com/actix/examples/blob/master/async_db/src/main.rs
    state
        .db
        .send(CheckGpioLevel {
            gpio_id: gpio_id,
            gpio_level: gpio_level,
        })
        .from_err()
        .or_else(|err: Error | Ok(HttpResponse::InternalServerError()
            .body("feilure!!!!!!!")
            .into())
            );
        /*
        .from_err()
        .and_then(|res| match res {
            Ok(_) => {},
            Err(err) => Ok(HttpResponse::InternalServerError()
                .body(err.to_string())
                .into()),
        });
    */

    /*
    state
        .db
        .send
    */

    state
        .db
        .send(SetGpioLevel {
            gpio_id: gpio_id,
            gpio_level: gpio_level,
        })
        .from_err()
        .and_then(|res| match res {
            Ok(response) => Ok(HttpResponse::Ok().json(response)),
            Err(err) => Ok(HttpResponse::InternalServerError()
                .body(err.to_string())
                .into()),
        })
        .responder()

    // set_gpio_level_rpi(gpio_id, &gpio_level, state.gpio_arc_mutex.clone());
}

/// creates and returns the app after mounting all routes/resources
pub fn create_app(app_state: AppState) -> App<AppState> {
    App::with_state(app_state)
        // enable logger
        .middleware(middleware::Logger::default())
        .resource("/status/{id}", |r| {
            r.method(http::Method::GET).with(gpio_status)
        })
        .resource("/set_level/{id}/{level}", |r| {
            r.method(http::Method::GET).with(set_gpio_level)
        })
}
