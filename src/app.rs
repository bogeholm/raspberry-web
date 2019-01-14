use std::sync::Arc;
use parking_lot::Mutex;
use actix::{Addr};
use actix_web::Error;
use actix_web::{http, middleware, App, AsyncResponder, FutureResponse, HttpResponse, Path, State}; //Error};
use futures::{Future, future};//, future::FutureResult};
use crate::handlers::{DbExecutor, GpioId, CheckGpioLevel, SetGpioLevel};
use crate::models;
use crate::rpi;

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
    (req, state): (Path<(i32, String)>, State<AppState>)) -> FutureResponse<HttpResponse> {

    // https://github.com/actix/examples/blob/master/async_db/src/main.rs
    // https://github.com/actix/examples/blob/master/actix_todo/src/api.rs
    // https://stackoverflow.com/questions/54164682/
    state
    .db
    .send(CheckGpioLevel {
            gpio_id: req.0,
            gpio_level: req.1.clone(),
    })
    .from_err()
    .and_then(|res| future::result(res).from_err())
    .and_then(move |_res| {
        // Do some additional logic here
        let _level_updated = rpi::set_gpio_level_rpi(req.0, &req.1.clone(), state.gpio_arc_mutex.clone());

        // Update database to correspond with above
        state
        .db
        .send(SetGpioLevel {
            gpio_id: req.0,
            gpio_level: req.1.clone(),
        })
        .from_err()
    })
    .and_then(|res| future::result(res).from_err())
    .then(|res: Result<models::Gpio, Error>| match res {
       Ok(response) => Ok(HttpResponse::Ok().json(response)),
       Err(err) => Ok(HttpResponse::InternalServerError().body(err.to_string()))
      })
    .responder()
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
