use crate::handlers::{CheckGpioLevel, DbExecutor, GpioId, SetGpioLevel};
use crate::models;
use crate::rpi;
use actix::Addr;
use actix_web::Error;
use actix_web::{http, middleware, App, AsyncResponder, FutureResponse, HttpResponse, Path, State}; //Error};
use futures::{future, Future}; //, future::FutureResult};
use parking_lot::Mutex;
use std::sync::Arc;

#[cfg(target_arch = "armv7")]
use rppal::gpio::{Error::InstanceExists, Gpio};

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
    pub gpio_arc_mutex: Arc<Mutex<Gpio>>,
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
    (req, state): (Path<(i32, String)>, State<AppState>),
) -> FutureResponse<HttpResponse> {
    let path_gpio_id: i32 = req.0;
    let path_gpio_level = req.1.clone();
    let path_gpio_level_copy = req.1.clone(); // TODO - OMG this is horrible
    let gpio_arc_mutex = state.gpio_arc_mutex.clone();

    // https://github.com/actix/examples/blob/master/async_db/src/main.rs
    // https://github.com/actix/examples/blob/master/actix_todo/src/api.rs
    // https://stackoverflow.com/questions/54164682/
    state
        .db
        .send(CheckGpioLevel {
            gpio_id: path_gpio_id,
            gpio_level: (&path_gpio_level).to_string(),
        })
        .from_err()
        .and_then(|res| future::result(res).from_err())
        .and_then(move |_| {
            // Update GPIO level on RPi
            let level_updated =
                rpi::set_gpio_level_rpi(path_gpio_id, &path_gpio_level, gpio_arc_mutex);
            future::result(level_updated).from_err()
        })
        .and_then(move |_| {
            // Update database to correspond with above
            state
                .db
                .send(SetGpioLevel {
                    gpio_id: path_gpio_id,
                    gpio_level: (&path_gpio_level_copy).to_string(),
                })
                .from_err()
        })
        .and_then(|res| future::result(res).from_err())
        .then(|res: Result<models::Gpio, Error>| match res {
            Ok(response) => Ok(HttpResponse::Ok().json(response)),
            Err(err) => Ok(HttpResponse::InternalServerError().body(err.to_string())),
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
